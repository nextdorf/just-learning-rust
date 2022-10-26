#include "VideoStream.h"
#include <assert.h>

int vs_seek(struct VideoStream *stream, int64_t timestamp, int flags, bool decode_frames, int* err){
  *err = 0;
  if(!stream->fmt_ctx) return vs_fmt_ctx_is_none;
  if(!stream->stream) return vs_stream_is_none;

  if(decode_frames)
    flags |= AVSEEK_FLAG_BACKWARD;
  flags |= stream->fmt_ctx->flags;
  *err = av_seek_frame(stream->fmt_ctx, stream->stream->index, timestamp, flags);
  if(*err < 0) return vs_ffmpeg_errorcode;

  if(decode_frames) {
    if(!stream->pkt)
      stream->pkt = av_packet_alloc();
    if(!stream->frm)
      stream->frm = av_frame_alloc();

    int res = vs_success;
    while(stream->frm->pts == AV_NOPTS_VALUE || timestamp >= stream->frm->pts + stream->frm->pkt_duration) {
      res = vs_decode_next_frame(stream, false, err);
      if(res != vs_success && res != vs_eof)
        return res;
    }

    if(timestamp > stream->frm->pts + stream->frm->pkt_duration)
      return vs_timestamp_out_of_bounds;
    if(stream->frm->pts == AV_NOPTS_VALUE)
      return vs_ffmpeg_errorcode;

  }

  return vs_success;
}

int vs_seek_at(struct VideoStream *stream, double seconds, int flags, bool read_packets, int *err){
  if(!stream->fmt_ctx) return vs_fmt_ctx_is_none;
  if(!stream->stream) return vs_stream_is_none;

  int64_t timestamp = seconds*stream->stream->time_base.den/stream->stream->time_base.num;
  return vs_seek(stream, timestamp, flags, read_packets, err);
}

int vs_decode_frames(struct VideoStream *stream, bool invoke_sws, int *err, uint32_t nFrames){
  if(   nFrames > 0
    ||  stream->pkt->dts != stream->frm->pkt_dts
    // ||  stream->pkt->pts != stream->frm->pts
    ||  stream->frm->pkt_dts == AV_NOPTS_VALUE ) {
    while(true){
      if(nFrames > 0){
        av_packet_unref(stream->pkt);
        *err = av_read_frame(stream->fmt_ctx, stream->pkt);
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
        if(stream->pkt->stream_index != stream->stream->index)
          continue;

        *err = avcodec_send_packet(stream->codec_ctx, stream->pkt);
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
            if(av_codec_is_encoder(stream->codec))
              return vs_encoder_trys_to_decode;
            else
              return vs_ffmpeg_errorcode;
          default:
            return vs_ffmpeg_errorcode;
        }
      }

      av_frame_unref(stream->frm);
      *err = avcodec_receive_frame(stream->codec_ctx, stream->frm);
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
  if(invoke_sws && stream->swsfrm->pts != stream->frm->pts){
    av_frame_unref(stream->swsfrm);
    if((*err = sws_scale_frame(stream->sws_ctx, stream->swsfrm, stream->frm)) < 0)
      return vs_ffmpeg_errorcode;
    stream->swsfrm->best_effort_timestamp = stream->frm->best_effort_timestamp;
    stream->swsfrm->pts = stream->frm->pts;
    stream->swsfrm->pkt_dts = stream->frm->pkt_dts;
    stream->swsfrm->pkt_duration = stream->frm->pkt_duration;
  }
  return vs_success;
}

int vs_decode_current_frame(struct VideoStream *stream, bool invoke_sws, int *err){
  return vs_decode_frames(stream, invoke_sws, err, 0);
}

int vs_decode_next_frame(struct VideoStream *stream, bool invoke_sws, int *err){
  return vs_decode_frames(stream, invoke_sws, err, 1);
}


