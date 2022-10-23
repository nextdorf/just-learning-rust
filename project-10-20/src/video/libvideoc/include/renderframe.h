#ifndef RENDERFRAME_H
#define RENDERFRAME_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>


int renderfrom(const char *path, 
  char* data[8], int* width, int* height, int linspace[8], 
  const int skip_frames);

#endif
