use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeError {
    Validation,
    NotFound,
    NotNative,
    Conflict,
    Busy,
    Unavailable,
    Authentication,
    Forbidden,
    UpstreamAuthentication,
    Tls,
    Upstream,
    Storage,
}

impl BridgeError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation => "validation_error",
            Self::NotFound => "not_found",
            Self::NotNative => "not_native",
            Self::Conflict => "conflict",
            Self::Busy => "busy",
            Self::Unavailable => "unavailable",
            Self::Authentication => "authentication_required",
            Self::Forbidden => "forbidden",
            Self::UpstreamAuthentication => "upstream_authentication_failed",
            Self::Tls => "tls_validation_failed",
            Self::Upstream => "upstream_error",
            Self::Storage => "storage_error",
        }
    }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Validation => "Invalid value or capability.",
            Self::NotFound => "The requested resource was not found.",
            Self::NotNative => "This device is not in native mode.",
            Self::Conflict => "The request conflicts with existing state.",
            Self::Busy => "The service is busy.",
            Self::Unavailable => "The device or gateway is unavailable.",
            Self::Authentication => "Authentication is required.",
            Self::Forbidden => "Permission denied.",
            Self::UpstreamAuthentication => "Manufacturer authentication failed.",
            Self::Tls => "TLS validation failed.",
            Self::Upstream => "The manufacturer response could not be used.",
            Self::Storage => "Storage operation failed.",
        }
    }
}
impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}
impl std::error::Error for BridgeError {}
