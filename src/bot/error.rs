use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ErrZalo {
    BadRequest = 400,
    Unauthorized = 401,
    InternalServerError = 403,
    NotFound = 404,
    RequestTimeout = 408,
    QuotaExceeded = 429,
    Unknown = 0,
}

impl ErrZalo {
    pub fn from_code(code: i32) -> Self {
        match code {
            400 => ErrZalo::BadRequest,
            401 => ErrZalo::Unauthorized,
            403 => ErrZalo::InternalServerError,
            404 => ErrZalo::NotFound,
            408 => ErrZalo::RequestTimeout,
            429 => ErrZalo::QuotaExceeded,
            _ => ErrZalo::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ErrZalo::BadRequest => "Bad request - sai đường dẫn hoặc API Name không hợp lệ",
            ErrZalo::Unauthorized => "Unauthorized - Token đã hết hạn hoặc không hợp lệ",
            ErrZalo::InternalServerError => "Internal server error",
            ErrZalo::NotFound => "Not found - Yêu cầu truy cập không lệ",
            ErrZalo::RequestTimeout => "Request timeout - Quá thời gian xử lý cho phép",
            ErrZalo::QuotaExceeded => "Quota exceeded - Vượt quá giới hạn sử dụng API cho phép",
            ErrZalo::Unknown => "Unknown error",
        }
    }
}

impl fmt::Display for ErrZalo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Code {}: {}", *self as i32, self.as_str())
    }
}

impl std::error::Error for ErrZalo {}
