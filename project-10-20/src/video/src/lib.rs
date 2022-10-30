mod ffi;
pub use ffi::VideoStream;

pub enum VideoStreamErr{
  FFMPEGErr {err: i32},
  TimeStampOutOfBounds,
  EOF,
  IO,
  EncoderTrysToDecode,
  DecoderTrysToEncode,
  IndexOutOfBounds,
  StreamNotFound,
  DecoderNotFound,
}

fn wrap_VSResult<T>(res: ffi::VideoStreamResult, err: i32, x: T) -> Result<T, VideoStreamErr> {
  match res {
    ffi::VideoStreamResult::vs_ffmpeg_errorcode => Err(VideoStreamErr::FFMPEGErr { err }),
    ffi::VideoStreamResult::vs_success => Ok(x),
    ffi::VideoStreamResult::vs_timestamp_out_of_bounds => Err(VideoStreamErr::TimeStampOutOfBounds),
    ffi::VideoStreamResult::vs_eof => Err(VideoStreamErr::EOF),
    ffi::VideoStreamResult::vs_io => Err(VideoStreamErr::IO),
    ffi::VideoStreamResult::vs_encoder_trys_to_decode => Err(VideoStreamErr::EncoderTrysToDecode),
    ffi::VideoStreamResult::vs_decoder_trys_to_encode => Err(VideoStreamErr::DecoderNotFound),
    ffi::VideoStreamResult::vs_index_out_of_bounds => Err(VideoStreamErr::IndexOutOfBounds),
    ffi::VideoStreamResult::vs_stream_not_found => Err(VideoStreamErr::StreamNotFound),
    ffi::VideoStreamResult::vs_decoder_not_found => Err(VideoStreamErr::DecoderNotFound),
  }
}

impl VideoStream{
  fn is_valid(self) -> bool{
    false
  }
}

pub struct PartialVideoStream {
  val: VideoStream
}

impl Into<Option<VideoStream>> for PartialVideoStream {
  fn into(self) -> Option<VideoStream> {
    match self.val.is_valid() {
      true => Some(self.val),
      false => None
    }
  }
}

