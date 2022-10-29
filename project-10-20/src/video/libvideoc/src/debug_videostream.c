#include "VideoStream.h"
#include <stdio.h>
#include <stdint.h>

int my_renderframe(char *path, double seconds, int *width, int *height, int *outwidth, int *outheight, uint8_t **rgb);

double parseTimeInSecs(const char *s);

void dumbFctxInfo(AVFormatContext *fctx, int defaultIdx);

int main(int argc, char** argv){
  const double skip_secs_default = 10;

  int width = -1, height = -1, outwidth = 1280, outheight = 720;
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

  int res = my_renderframe(path, skip_secs, &width, &height, &outwidth, &outheight, &data);

  printf("Result: %i\n  decoded size:\t %ix%i\n  final size:\t %ix%i\n", res, width, height, outwidth, outheight);


  // FILE *f = fopen("raw.rgb", "wb");
  // size_t nWrote = fwrite(data, 3, outwidth*outheight, f);
  // fclose(f);
  // free(data);

  return res;
}

int my_renderframe(char *path, double seconds, int *width, int *height, int *outwidth, int *outheight, uint8_t **rgb){
  AVFormatContext *fctx = NULL;
  AVCodecContext *cctx = NULL;
  struct SwsContext *swsctx = NULL;
  int err;

  open_format_context_from_path(path, &fctx, &err);
  int idx = av_find_default_stream_index(fctx);

  // dumbFctxInfo(fctx, idx);

  open_codec_context(fctx, idx, &cctx, &err);

  *width = cctx->width;
  *height = cctx->height;
  if(*outwidth < 0) *outwidth = *width;
  if(*outheight < 0) *outheight = *height;
  create_sws_context(cctx, &swsctx, *outwidth, *outheight, AV_PIX_FMT_RGB24, SWS_SPLINE, NULL, &err);


  VideoStream *vstream = malloc(sizeof(VideoStream));
  vstream->fmt_ctx = fctx;
  vstream->codec_ctx = cctx;
  vstream->stream = fctx->streams[idx];
  vstream->pkt = av_packet_alloc();
  vstream->frm = av_frame_alloc();
  vstream->swsfrm = av_frame_alloc();
  vstream->sws_ctx = swsctx;


  vs_seek_at(vstream->fmt_ctx, vstream->stream, seconds, 0, vstream->codec_ctx, vstream->pkt, vstream->frm, &err);
  // vs_decode_current_frame(vstream->fmt_ctx, vstream->codec_ctx, vstream->stream, vstream->pkt, vstream->frm, vstream->sws_ctx, vstream->swsfrm, &err);
  // const size_t datasize = vstream->swsfrm->height * vstream->swsfrm->linesize[0];
  // *rgb = malloc(datasize);
  // memcpy(*rgb, vstream->swsfrm->data[0], datasize);


  av_frame_unref(vstream->frm);
  av_frame_unref(vstream->swsfrm);
  av_frame_free(&(vstream->frm));
  av_frame_free(&(vstream->swsfrm));
  av_packet_unref(vstream->pkt);
  av_packet_free(&vstream->pkt);
  sws_freeContext(vstream->sws_ctx);
  avcodec_close(vstream->codec_ctx);
  avformat_close_input(&(vstream->fmt_ctx));
  free(vstream);

  return err;
}

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

char *parseTimeFromSecs(double t){
  const char invalidStr[] = "???";
  char *ret = malloc(1024);
  if(t < 0){
    strcpy(ret, invalidStr);
    return ret;
  }
  uint64_t tInt = t;
  uint64_t h = tInt/(60*60), m = (tInt % (60*60))/(60);
  double s = (tInt % 60) + (t - tInt);
  const char fmtFull[] = "%ih%02im%02.2fs";
  if(h == 0){
    if(m == 0)
      sprintf(ret, fmtFull + 3 + 5, s);
    else
      sprintf(ret, fmtFull + 3, m, s);
  }
  else
    sprintf(ret, fmtFull, h, m, s);
  return ret;
}

void dumbFctxInfo(AVFormatContext *fctx, int defaultIdx){
  printf("AVFormatContext:\n");

  printf("  Streams (len = %i)\n", fctx->nb_streams);
  for(int i=0; i<fctx->nb_streams; i++){
    AVStream *s = fctx->streams[i];
    double timebase = ((double)s->time_base.num)/s->time_base.den;
    double dur = timebase * s->duration;
    char *durStr = parseTimeFromSecs(dur);
    double startTime = timebase * s->start_time;
    int streamID = s->id;
    int64_t nFrames = s->nb_frames ? s->nb_frames : (s->duration ? -1 : 0);
    printf("  %s%i:\t nFrames = %i, dur = %s\n", (i==defaultIdx) ? "->":"  ", i, nFrames, durStr);
    free(durStr);
    AVDictionaryEntry *m = av_dict_get(s->metadata, "", NULL, AV_DICT_IGNORE_SUFFIX);
    if(m) {
      printf("      \t metadata = { %s: %s", m->key, m->value);
      while(m = av_dict_get(s->metadata, "", m, AV_DICT_IGNORE_SUFFIX))
        printf(", %s: %s", m->key, m->value);
      printf(" }\n");
    }
  }

  printf("  Programs (len = %i)\n", fctx->nb_programs);
  printf("  Chapters (len = %i)\n", fctx->nb_chapters);
  for(int i=0; i<fctx->nb_chapters; i++){
    AVChapter *ch = fctx->chapters[i];
    double timebase = ((double)ch->time_base.num)/ch->time_base.den;
    char *t1 = parseTimeFromSecs(timebase * ch->start), *t2 = parseTimeFromSecs(timebase * ch->end);
    int chapterID = ch->id;
    printf("    %i:\t dur = %s - %s\n", i, t1, t2);
    AVDictionaryEntry *m = av_dict_get(ch->metadata, "", NULL, AV_DICT_IGNORE_SUFFIX);
    if(m) {
      printf("      \t metadata = { %s: %s", m->key, m->value);
      while(m = av_dict_get(ch->metadata, "", m, AV_DICT_IGNORE_SUFFIX))
        printf(", %s: %s", m->key, m->value);
      printf(" }\n");
    }
  }
}

