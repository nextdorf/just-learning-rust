#include "renderframe.h"

int renderfrom(const char *path, 
  char* data[8], int* width, int* height, int linesize[8], 
  const int skip_frames) {
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
  int idx = 0;
  AVStream *stream = fctx->streams[idx];
  idx = av_find_best_stream(fctx, stream->codecpar->codec_type, idx, -1, (const AVCodec**)&codec, 0);
  stream = fctx->streams[idx];

  cctx = avcodec_alloc_context3(codec);
  avcodec_parameters_to_context(cctx, stream->codecpar);
  avcodec_open2(cctx, codec, NULL);


  AVPacket *pkt = av_packet_alloc();
  if(!pkt) return 0;
  AVFrame *frm = av_frame_alloc();
  if(!frm) {av_packet_free(&pkt); return 0;}

  uint8_t *ret = NULL;
  int skipFrames = skip_frames;
  while(!ret && (err = av_read_frame(fctx, pkt)) >= 0){
    if(pkt->stream_index != idx)
      continue;
    if((err = avcodec_send_packet(cctx, pkt)) < 0)
      break;

    err = avcodec_receive_frame(cctx, frm);
    if(err >= 0){
      if(skipFrames > 0)
        --skipFrames;
      else{
        *width = frm->width;
        *height = frm->height;
        for(int i=0; i<8; ++i) {
          linesize[i] = frm->linesize[i];
          if (linesize[i]){
            const size_t n = linesize[i] * *height;
            data[i] = malloc(n);
            memcpy(data[i], frm->data[i], n);
          }
          else
            data[i] = NULL;
        }
        break;
      }
      //ret = frm->data;
    }
    else if(err == AVERROR(EAGAIN) || err == AVERROR_EOF)
      continue;
    else
      break;
  }

  if(skipFrames > 0)
    err = -1;

  av_frame_free(&frm);
  av_packet_free(&pkt);
  avformat_close_input(&fctx);

  return err;
}

/*
void dumpData(AVFrame *frm, const char *path){
  FILE* f = fopen( path, "wb");
  int32_t header[] = {frm->width, frm->height, frm->linesize[0], frm->linesize[1], frm->linesize[2]};

  //fwrite(header, sizeof(*header), sizeof(header)/sizeof(*header), f);
  for(int i = 0; i<3; ++i){
    const size_t towrite = frm->height*frm->linesize[i];
    size_t written = fwrite(frm->data[i], 1, towrite, f);
    printf("%i: wrote %i / %i bytes\n", i, written, towrite);
  }
  fclose(f);
}
*/


