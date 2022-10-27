#include "VideoStream.h"
#include <assert.h>


VideoStreamResult open_format_context_from_path(char *path, AVFormatContext **fmt_ctx, int *err){
  if((*err = avformat_open_input(fmt_ctx, path, NULL, NULL)) < 0)
    return vs_ffmpeg_errorcode;
  if((*err = avformat_find_stream_info(*fmt_ctx, NULL)) < 0)
    return vs_io;
  return vs_success;
}

VideoStreamResult open_codec_context(AVFormatContext *fmt_ctx, int stream_idx, AVCodecContext **codec_ctx, int *err){
  /*
  Let Rust call av_find_default_stream_index, av_find_best_stream in any way and specify the behaivour with enums

  Programs are essentially a "bundle of streams" (see https://en.wikipedia.org/wiki/MPEG_transport_stream#Programs) and by
  specifying related stream in av_find_best_stream FFMPEG trys to find a stream in the same program (https://ffmpeg.org/doxygen/5.1/avformat_8c_source.html#l00347)
  */
  if(stream_idx < 0 || stream_idx >= fmt_ctx->nb_streams)
    return vs_index_out_of_bounds;
  AVStream *stream = fmt_ctx->streams[stream_idx];
  AVCodec *codec = NULL;
  // Internal ff_find_decoder() is not part of public ABI
  *err = av_find_best_stream(fmt_ctx, stream->codecpar->codec_type, stream_idx, -1, (const AVCodec**)&codec, 0);
  if(*err != stream_idx)
    switch(*err){
      case AVERROR_DECODER_NOT_FOUND: //AVERROR_DECODER_NOT_FOUND if streams were found but no decoder
        return vs_decoder_not_found;
      case AVERROR_STREAM_NOT_FOUND: //AVERROR_STREAM_NOT_FOUND if no stream with the requested type could be found
      default: //wrong stream found(?)
        return vs_stream_not_found;
    }

  *codec_ctx = avcodec_alloc_context3(codec);
  if((*err = avcodec_parameters_to_context(*codec_ctx, stream->codecpar)) < 0)
    return vs_ffmpeg_errorcode;
  if((*err = avcodec_open2(*codec_ctx, codec, NULL)) < 0)
    return vs_ffmpeg_errorcode;

  // If the following asserts hold there is need for exporting the codec
  // assert((*codec_ctx)->codec->id == codec->id);
  assert(!strcmp((*codec_ctx)->codec->name, codec->name));
  assert(av_codec_is_encoder((*codec_ctx)->codec) == av_codec_is_encoder(codec));
  assert(av_codec_is_decoder((*codec_ctx)->codec) == av_codec_is_decoder(codec));

  return vs_success;
}

VideoStreamResult create_sws_context(AVCodecContext *codec_ctx, struct SwsContext **sws_ctx,
  int new_width, int new_height, enum AVPixelFormat new_pix_fmt, int flags, const double *param, int *err){
  *sws_ctx = sws_getContext(
    codec_ctx->width, codec_ctx->height, codec_ctx->pix_fmt, 
    new_width, new_height, new_pix_fmt,
    flags, NULL, NULL, param );
  return *sws_ctx ? vs_success : vs_ffmpeg_errorcode;
}



VideoStreamResult vs_seek(AVFormatContext *fmt_ctx, AVStream *stream, int64_t timestamp, int flags, AVCodecContext *codec_ctx_if_decode_frames, AVPacket *pkt, AVFrame *frm, int* err){
  *err = 0;

  if(flags >= 0){
    // Fast mode
    if(codec_ctx_if_decode_frames)
      flags |= AVSEEK_FLAG_BACKWARD;
    flags |= fmt_ctx->flags;
    *err = av_seek_frame(fmt_ctx, stream->index, timestamp, flags);
    if(*err < 0) return vs_ffmpeg_errorcode;
  }

  if(codec_ctx_if_decode_frames) {
    // Precise mode
    int res = vs_success;
    while(frm->pts == AV_NOPTS_VALUE || timestamp >= frm->pts + frm->pkt_duration) {
      res = vs_decode_next_frame(fmt_ctx, codec_ctx_if_decode_frames, stream, pkt, frm, NULL, NULL, err);
      if(res != vs_success && res != vs_eof)
        return res;
    }

    if(timestamp > frm->pts + frm->pkt_duration)
      return vs_timestamp_out_of_bounds;
    if(frm->pts == AV_NOPTS_VALUE)
      return vs_ffmpeg_errorcode;
  }
  return vs_success;
}

VideoStreamResult vs_seek_at(AVFormatContext *fmt_ctx, AVStream *stream, double seconds, int flags, AVCodecContext *codec_ctx_if_decode_frames, AVPacket *pkt, AVFrame *frm, int* err){
  const int64_t timestamp = seconds * stream->time_base.den/stream->time_base.num;
  return vs_seek(fmt_ctx, stream, timestamp, flags, codec_ctx_if_decode_frames, pkt, frm, err);
}

VideoStreamResult vs_decode_frames(AVFormatContext *fmt_ctx, AVCodecContext *codec_ctx, AVStream *stream, AVPacket *pkt, AVFrame *frm, struct SwsContext *sws_ctx_if_scale, AVFrame *swsfrm, uint32_t nFrames, int *err){
  if(   nFrames > 0
    ||  pkt->dts != frm->pkt_dts
    // ||  pkt->pts != frm->pts
    ||  frm->pkt_dts == AV_NOPTS_VALUE ) {
    while(true){
      if(nFrames > 0){
        av_packet_unref(pkt);
        *err = av_read_frame(fmt_ctx, pkt);
        switch(*err){
          case 0:
            break;
          // //Should never happen
          // case AVERROR(EAGAIN):
          //   continue;
          case AVERROR_EOF:
            return vs_eof;
          default:
            return vs_ffmpeg_errorcode;
        }

        // //Used to continue if this is false (copied from a tutorial), but there more I think about it the less sense it makes
        // assert(stream->pkt->stream_index == stream->stream->index);
        //Yeah the index does indeed change, not sure what it means tho. Sound catching up?
        if(pkt->stream_index != stream->index)
          continue;

        *err = avcodec_send_packet(codec_ctx, pkt);
        switch(*err){
          case AVERROR(EAGAIN): //input is not accepted in the current state - user must read output with avcodec_receive_frame() (once all output is read, the packet should be resent, and the call will not fail with EAGAIN).
            // noop in switch, instead of continueing the loop because this errorcode means here
            // that the frame was only partially read, i.e. the packet was already sent to codec_ctx
            // but wasn't received yet by the frame
            // TODO: Actually check if the above comment is true and we shouldn't do: avcodec_receive_frame -> avcodec_send_packet -> break switch
          case 0:
            break;
          case AVERROR(AVERROR_EOF): //the decoder has been flushed, and no new packets can be sent to it (also returned if more than 1 flush packet is sent)
            return vs_eof;
          case AVERROR(EINVAL): //codec not opened, it is an encoder, or requires flush
            if(av_codec_is_encoder(codec_ctx->codec))
              return vs_encoder_trys_to_decode;
            else
              return vs_ffmpeg_errorcode;
          default:
            return vs_ffmpeg_errorcode;
        }
      }

      av_frame_unref(frm);
      *err = avcodec_receive_frame(codec_ctx, frm);
      switch (*err) {
        case 0:
          if(--nFrames > 0)
            continue;
          break;
        case AVERROR(EAGAIN): // output is not available in this state - user must try to send new input
          if(nFrames == 0)
            ++nFrames;
          continue;
        case AVERROR_EOF: // he decoder has been fully flushed, and there will be no more output frames
          return vs_eof;
        default:
          // AVERROR(EINVAL): codec not opened, or it is an encoder
          // AVERROR_INPUT_CHANGED: current decoded frame has changed parameters with respect to first decoded frame. Applicable when flag AV_CODEC_FLAG_DROPCHANGED is set.
          // other negative values: legitimate decoding errors
          return vs_ffmpeg_errorcode;
      }
      break; // err = 0, success
    }
  }
  if(sws_ctx_if_scale && swsfrm->pts != frm->pts){
    av_frame_unref(swsfrm);
    *err = sws_scale_frame(sws_ctx_if_scale, swsfrm, frm);
    if(*err < 0)
      return vs_ffmpeg_errorcode;
    swsfrm->best_effort_timestamp = frm->best_effort_timestamp;
    swsfrm->pts = frm->pts;
    swsfrm->pkt_dts = frm->pkt_dts;
    swsfrm->pkt_duration = frm->pkt_duration;
  }
  return vs_success;
}

VideoStreamResult vs_decode_current_frame(AVFormatContext *fmt_ctx, AVCodecContext *codec_ctx, AVStream *stream, AVPacket *pkt, AVFrame *frm, struct SwsContext *sws_ctx, AVFrame *swsfrm, int *err){
  return vs_decode_frames(fmt_ctx, codec_ctx, stream, pkt, frm, sws_ctx, swsfrm, 0, err);
}

VideoStreamResult vs_decode_next_frame(AVFormatContext *fmt_ctx, AVCodecContext *codec_ctx, AVStream *stream, AVPacket *pkt, AVFrame *frm, struct SwsContext *sws_ctx, AVFrame *swsfrm, int *err){
  return vs_decode_frames(fmt_ctx, codec_ctx, stream, pkt, frm, sws_ctx, swsfrm, 1, err);
}


