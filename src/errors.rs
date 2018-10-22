//! Error types & handling.
pub type MediaResult<T> = Result<T, MediaError>;

#[derive(Debug, Fail)]
pub enum MediaError {
    #[fail(display = "Error code {} in {}", code, from)]
    UnknownErrorCode {
        from: &'static str,
        code: i32
    },
    #[fail(display = "Encountered a NUL byte in a Rust &str")]
    NulError,
    #[fail(display = "Failed to find an audio stream in the given file.")]
    StreamNotFound,
    #[fail(display = "Failed to find a decoder for the audio stream.")]
    DecoderNotFound,
    #[fail(display = "An allocation failed.")]
    AllocationFailed,
    #[fail(display = "Resource temporarily unavailable (EAGAIN).")]
    TemporarilyUnavailable,
    #[fail(display = "Programmer error: this should never happen (possibly EINVAL).")]
    ProgrammerError,
    #[fail(display = "End of file.")]
    EOF,
    #[fail(display = "Device or resource busy (EBUSY).")]
    DevBusy,
    #[fail(display = "Input/output error (EIO).")]
    IOError,
    #[fail(display = "Permission denied (EACCES).")]
    PermissionDenied,
    #[fail(display = "Is a directory (EISDIR).")]
    IsADir,
    #[fail(display = "The file's sample format is currently unsupported.")]
    UnsupportedFormat,
    #[fail(display = "You may only call that function once.")]
    OnceOnly,
    #[fail(display = "You specified a duration that is larger than 9,223,372,036,854,775,807μs.")]
    TooManySeconds,
    #[fail(display = "FFmpeg error: Bitstream filter not found")]
    BsfNotFound,
    #[fail(display = "FFmpeg error: Internal bug")]
    FfmpegBug,
    #[fail(display = "FFmpeg error: Buffer too small")]
    BufferTooSmall,
    #[fail(display = "FFmpeg error: Decoder not found")]
    DemuxerNotFound,
    #[fail(display = "FFmpeg error: Demuxer not found")]
    EncoderNotFound,
    #[fail(display = "FFmpeg error: Encoder not found")]
    ExitRequested,
    #[fail(display = "FFmpeg error: End of file")]
    ExternalError,
    #[fail(display = "FFmpeg error: Filter not found")]
    FilterNotFound,
    #[fail(display = "FFmpeg error: Invalid data found when processing input")]
    InvalidData,
    #[fail(display = "FFmpeg error: Muxer not found")]
    MuxerNotFound,
    #[fail(display = "FFmpeg error: Option not found")]
    OptionNotFound,
    #[fail(display = "FFmpeg error: Not yet implemented in FFmpeg, patches welcome")]
    PatchWelcome,
    #[fail(display = "FFmpeg error: Protocol not found")]
    ProtocolNotFound,
    #[fail(display = "FFmpeg error: Unknown error, typically from an external library")]
    FfmpegUnknown,
    #[fail(display = "FFmpeg error: Requested feature is flagged experimental. Set strict_std_compliance if you really want to use it.")]
    FeatureExperimental,
    #[fail(display = "No such file or directory (ENOENT).")]
    FileNotFound,
    #[fail(display = "FFmpeg error: HTTP 400 Bad Request")]
    HttpBadRequest,
    #[fail(display = "FFmpeg error: HTTP 401 Unauthorized")]
    HttpUnauthorized,
    #[fail(display = "FFmpeg error: HTTP 404 Not Found")]
    HttpNotFound,
    #[fail(display = "FFmpeg error: HTTP 403 Forbidden")]
    HttpForbidden,
    #[fail(display = "FFmpeg error: Other HTTP 4xx error")]
    HttpOther4xx,
    #[fail(display = "FFmpeg error: HTTP 5xx Server Error")]
    HttpServerError,
}
