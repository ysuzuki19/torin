mod macros;

use backtrace::Backtrace;
use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
    #[cfg(debug_assertions)]
    backtrace: Backtrace,
}

impl Error {
    pub fn new<S>(message: S) -> Self
    where
        S: AsRef<str>,
    {
        Error {
            message: message.as_ref().to_string(),
            #[cfg(debug_assertions)]
            backtrace: Backtrace::new(),
        }
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error,
{
    fn from(err: E) -> Self {
        Error::new(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "message: {}", self.message)?;

        #[cfg(debug_assertions)]
        {
            let filter = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
            for frame in self.backtrace.frames() {
                for symbol in frame.symbols() {
                    if let Some(file) = symbol.filename()
                        && file.to_string_lossy().starts_with(&filter)
                    {
                        let lineno = symbol.lineno().unwrap_or_default();
                        writeln!(f, "- {}:{}", file.display(), lineno)?;
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn with_trace<F>(f: F)
where
    F: Fn() -> Result<()>,
{
    if let Err(e) = f() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

pub mod testing {
    pub fn with_trace<F>(f: F)
    where
        F: Fn() -> super::Result<()>,
    {
        let res = f();
        assert!(
            res.is_ok(),
            "Function failed with error:\n {}",
            res.unwrap_err()
        );
    }
}
