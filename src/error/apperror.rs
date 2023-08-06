use std::fmt;

pub enum AppError {
    TestError,
    ChannelClosed,
    ChannelReceiveError,
    ChannelSendError,
    InternalError(String),
}

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TestError => write!(f, "just a test error, nothing anormal"),
            AppError::ChannelClosed => write!(f, "channel closed"),
            AppError::ChannelReceiveError =>  write!(f, "channel receive error"),
            AppError::ChannelSendError =>  write!(f, "channel send error"),
            AppError::InternalError(e) => write!(f, "internal error: {}", e),
        }
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}