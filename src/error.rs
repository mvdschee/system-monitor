use thiserror::Error as DisplayError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(DisplayError, Debug)]
pub enum Error {
	#[error("env key `{0}` is not set")]
	Env(String),

	#[error("main loop closed")]
	MainLoopClosed,

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("unknown error")]
	Unknown,
}
