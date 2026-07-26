use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ThreadError {
    InvalidThreadType(String),
    UnsupportedThreadSize {
        thread_type: String,
        size: String,
    },
    UnsupportedThreadPitch {
        thread_type: String,
        size: String,
        pitch: String,
    },
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadError::InvalidThreadType(thread_type) => {
                write!(f, "Unsupported thread type '{thread_type}'")
            }
            ThreadError::UnsupportedThreadSize { thread_type, size } => {
                write!(f, "Unsupported thread size '{size}' for {thread_type}")
            }
            ThreadError::UnsupportedThreadPitch {
                thread_type,
                size,
                pitch,
            } => {
                write!(
                    f,
                    "Unsupported pitch '{pitch}' for {thread_type} thread size {size}"
                )
            }
        }
    }
}

impl std::error::Error for ThreadError {}
