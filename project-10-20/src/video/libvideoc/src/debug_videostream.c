#include "VideoStream.h"
#include <stdio.h>
#include <stdint.h>

int my_renderframe(char *path, double seconds, int *width, int *height, uint8_t **rgb);

double parseTimeInSecs(const char *s){
  if(!s) return NAN;
  const size_t len = strlen(s);
  if(!len) return 0;

  char *tmp;
  double res;
  for(int i=0; i< len; i++){
    switch (s[i]) {
      case 's':
      case 'S':
        tmp = malloc(i+1);
        strncpy(tmp, s, i);
        tmp[i] = 0;
        res = atof(tmp);
        res += parseTimeInSecs(s + (i+1));
        free(tmp);
        return res;
      case 'm':
      case 'M':
        tmp = malloc(i+1);
        strncpy(tmp, s, i);
        tmp[i] = 0;
        res = atof(tmp)*60;
        res += parseTimeInSecs(s + (i+1));
        free(tmp);
        return res;
      case 'h':
      case 'H':
        tmp = malloc(i+1);
        strncpy(tmp, s, i);
        tmp[i] = 0;
        res = atof(tmp)*60*60;
        res += parseTimeInSecs(s + (i+1));
        free(tmp);
        return res;
      default:
        break;
    }
  }
  return atof(s);
}

int main(int argc, char** argv){
  const double skip_secs_default = 10;

  int width = -1, height = -1;
  char *path;
  uint8_t *data = NULL;
  double skip_secs = 0;

  if(argc <= 1 || strcmp(argv[1], "--help") == 0){
    printf("Usage: %s path-to-video-file [skip = %fs]\n", argv[0], skip_secs_default);
    return 0;
  }
  if(argc >= 2)
    path = argv[1];
  if(argc >= 3)
    for(int i=2; i<argc; ++i)
      skip_secs += parseTimeInSecs(argv[i]);
  else
    skip_secs = skip_secs_default;

  printf("Input:\n  Path:\t %s\n  skip:\t %.2f seconds\n", path, skip_secs);

  int res = my_renderframe(path, skip_secs, &width, &height, &data);

  printf("Result: %i\n  width:\t %i\n  height:\t %i\n", res, width, height);


  FILE *f = fopen("raw.rgb", "wb");
  size_t nWrote = fwrite(data, 1, 3*width*height, f);
  fclose(f);
  free(data);

  return res;
}

int my_renderframe(char *path, double seconds, int *width, int *height, uint8_t **rgb){
  AVFormatContext *fctx = NULL;
  AVCodecContext *cctx = NULL;
  int err;

  if ((err = avformat_open_input(&fctx, path, NULL, NULL)))
    return err;

  if ((err = avformat_find_stream_info(fctx, NULL)) < 0) {
    av_log(NULL, AV_LOG_ERROR, "Cannot find stream information\n");
    return err;
  }

  AVCodec *codec = NULL;
  int idx = av_find_default_stream_index(fctx);
  AVStream *stream = fctx->streams[idx];
  idx = av_find_best_stream(fctx, stream->codecpar->codec_type, idx, -1, (const AVCodec**)&codec, 0);
  if((err = idx) < 0)
    return err;
  stream = fctx->streams[idx];

  cctx = avcodec_alloc_context3(codec);
  avcodec_parameters_to_context(cctx, stream->codecpar);
  avcodec_open2(cctx, codec, NULL);


  VideoStream *vstream = malloc(sizeof(VideoStream));
  vstream->fmt_ctx = fctx;
  vstream->codec_ctx = cctx;
  vstream->codec = codec;
  vstream->stream = stream;
  vstream->pkt = av_packet_alloc();
  vstream->frm = av_frame_alloc();
  vstream->swsfrm = av_frame_alloc();
  vstream->sws_ctx = NULL;


  // vs_decode_current_frame(vstream, false, &err);
  vs_seek_at(vstream, seconds, 0, true, &err);
  *width = vstream->frm->width;
  *height = vstream->frm->height;
  vstream->sws_ctx = sws_getContext(
    *width, *height, vstream->frm->format, 
    *width, *height, AV_PIX_FMT_RGB24,
    SWS_BILINEAR, NULL, NULL, NULL );
  vs_decode_current_frame(vstream, true, &err);
  const size_t datasize = *height * vstream->swsfrm->linesize[0];
  *rgb = malloc(datasize);
  memcpy(*rgb, vstream->swsfrm->data[0], datasize);


  av_frame_unref(vstream->frm);
  av_frame_unref(vstream->swsfrm);
  av_frame_free(&(vstream->frm));
  av_frame_free(&(vstream->swsfrm));
  av_packet_unref(vstream->pkt);
  av_packet_free(&vstream->pkt);
  // avcodec_close(vstream->codec_ctx);
  sws_freeContext(vstream->sws_ctx);
  avformat_close_input(&(vstream->fmt_ctx));
  free(vstream);

  return err;
}


