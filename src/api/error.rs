use std::fmt;

/// Error type for all dawproject-rs operations.
#[derive(Debug)]
pub enum DawProjectError {
    /// I/O error (file not found, permission denied, etc.)
    Io(std::io::Error),
    /// XML serialization or deserialization error
    Xml(quick_xml::DeError),
    /// ZIP archive error
    Zip(zip::result::ZipError),
    /// Invalid or non-UTF-8 file path
    InvalidPath(String),
    /// Required file missing from the archive
    MissingFile(String),
}

impl fmt::Display for DawProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Xml(e) => write!(f, "XML error: {}", e),
            Self::Zip(e) => write!(f, "ZIP error: {}", e),
            Self::InvalidPath(p) => write!(f, "Invalid path: {}", p),
            Self::MissingFile(name) => write!(f, "Missing file in archive: {}", name),
        }
    }
}

impl std::error::Error for DawProjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Xml(e) => Some(e),
            Self::Zip(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DawProjectError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<quick_xml::DeError> for DawProjectError {
    fn from(e: quick_xml::DeError) -> Self {
        Self::Xml(e)
    }
}

impl From<zip::result::ZipError> for DawProjectError {
    fn from(e: zip::result::ZipError) -> Self {
        Self::Zip(e)
    }
}
