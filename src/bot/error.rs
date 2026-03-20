use std::fmt;

#[derive(Debug)]
pub enum ZaloError {
    Api(ErrZalo),
    Http(reqwest::Error),
    Json(serde_json::Error),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            ErrZalo::BadRequest => "Bad Request",
            ErrZalo::Unauthorized => "Unauthorized",
            ErrZalo::InternalServerError => "Internal Server Error",
            ErrZalo::NotFound => "Not Found",
            ErrZalo::RequestTimeout => "Request Timeout",
            ErrZalo::QuotaExceeded => "Quota Exceeded",
            ErrZalo::Unknown => "Unknown Error",
        }
    }
}

impl fmt::Display for ErrZalo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Zalo API Error (Code {}): {}",
            *self as i32,
            self.as_str()
        )
    }
}

impl fmt::Display for ZaloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZaloError::Api(e) => write!(f, "{}", e),
            ZaloError::Http(e) => write!(f, "HTTP Error: {}", e),
            ZaloError::Json(e) => write!(f, "JSON Error: {}", e),
            ZaloError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for ZaloError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ZaloError::Http(e) => Some(e),
            ZaloError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ZaloError {
    fn from(err: reqwest::Error) -> Self {
        ZaloError::Http(err)
    }
}

impl From<serde_json::Error> for ZaloError {
    fn from(err: serde_json::Error) -> Self {
        ZaloError::Json(err)
    }
}

impl From<ErrZalo> for ZaloError {
    fn from(err: ErrZalo) -> Self {
        ZaloError::Api(err)
    }
}
