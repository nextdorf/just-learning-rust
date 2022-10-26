#ifndef VIDEOSTREAM_H
#define VIDEOSTREAM_H


#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libswscale/swscale.h>
#include <libavutil/pixfmt.h>
#include <stdbool.h>


struct VideoStream
{
  AVFormatContext *fmt_ctx;
  AVCodec *codec;
  AVCodecContext *codec_ctx;
  AVStream *stream;

  AVPacket *pkt;
  AVFrame *frm;
  struct SwsContext *sws_ctx;
  AVFrame *swsfrm;
};

enum VideoStreamResult{
  vs_ffmpeg_errorcode = -1,
  vs_success = 0,
  vs_fmt_ctx_is_none,
  vs_codec_is_none,
  vs_codec_ctx_is_none,
  vs_stream_is_none,

  vs_timestamp_out_of_bounds,
  vs_eof,
  vs_encoder_trys_to_decode,
  vs_decoder_trys_to_encode,
};

typedef struct VideoStream VideoStream;

int vs_seek(struct VideoStream *stream, int64_t timestamp, int flags, bool read_packets, int *err);
int vs_seek_at(struct VideoStream *stream, double seconds, int flags, bool read_packets, int *err);

int vs_decode_current_frame(struct VideoStream *stream, bool invoke_sws, int* err);
int vs_decode_next_frame(struct VideoStream *stream, bool invoke_sws, int* err);
int vs_decode_frames(struct VideoStream *stream, bool invoke_sws, int *err, uint32_t nFrames);


#endif

