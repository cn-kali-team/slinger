//! engine error

use std::io::ErrorKind;
use std::num::ParseIntError;
#[cfg(feature = "tls")]
use socket2::Socket;
use thiserror::Error as ThisError;
/// A `Result` alias where the `Err` case is `slinger::Error`.
pub type Result<T> = std::result::Result<T, Error>;
/// The Errors that may occur when processing a `slinger`.
#[derive(ThisError, Debug)]
pub enum Error {
  #[cfg(feature = "tls")]
  /// HandshakeError
  #[error(transparent)]
  TlsHandshake(#[from] native_tls::HandshakeError<Socket>),
  #[error(transparent)]
  #[cfg(feature = "tls")]
  /// native_tls Error
  NativeTls(#[from] native_tls::Error),
  #[cfg(feature = "tls")]
  /// openssl ErrorStack
  #[error(transparent)]
  OpenSSl(#[from] openssl::error::ErrorStack),
  /// Error
  #[error(transparent)]
  IO(#[from] std::io::Error),
  /// http::Error
  #[error(transparent)]
  Http(http::Error),
  /// ParseIntError
  #[error(transparent)]
  IntError(#[from] ParseIntError),
  /// Proxy ReplyError
  #[error(transparent)]
  ReplyError(#[from] ReplyError),
  /// Unknown Error
  #[error("other")]
  Other(String),
}

#[derive(ThisError, Debug)]
pub enum ReplyError {
  #[error("Succeeded")]
  Succeeded,
  #[error("General failure")]
  GeneralFailure,
  #[error("Connection not allowed by ruleset")]
  ConnectionNotAllowed,
  #[error("Network unreachable")]
  NetworkUnreachable,
  #[error("Host unreachable")]
  HostUnreachable,
  #[error("Connection refused")]
  ConnectionRefused,
  #[error("TTL expired")]
  TtlExpired,
  #[error("Command not supported")]
  CommandNotSupported,
  #[error("Address type not supported")]
  AddressTypeNotSupported,
  //    OtherReply(u8),
}

impl From<http::Error> for Error {
  fn from(value: http::Error) -> Self {
    Error::Http(value)
  }
}

impl From<http::header::InvalidHeaderValue> for Error {
  fn from(value: http::header::InvalidHeaderValue) -> Self {
    Error::Http(http::Error::from(value))
  }
}

pub(crate) fn new_io_error(error_kind: ErrorKind, msg: &str) -> Error {
  Error::IO(std::io::Error::new(error_kind, msg))
}
