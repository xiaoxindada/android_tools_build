use std::fs::OpenOptions;
use std::io;
use std::os::unix::prelude::*;
use std::path::PathBuf;

/// Extension trait to [`OpenOptions`] to support opening files
/// in AIO mode
///
/// [`OpenOptions`]: ../tokio/fs/struct.OpenOptions.html
pub trait AioOpenOptionsExt {
    /// Opens a file at `path` with the options specified by `self` in direct mode
    /// for usage with AIO. It always adds `O_DIRECT` flag
    ///
    /// If `is_sync` is true, additionALLY `O_SYNC` flag will be added
    ///
    /// # Errors
    /// Error codes are the same as in the tokio version
    async fn aio_open(self, path: PathBuf, is_sync: bool) -> io::Result<crate::fs::File>;
}

impl AioOpenOptionsExt for OpenOptions {
    async fn aio_open(mut self, path: PathBuf, is_sync: bool) -> io::Result<crate::fs::File> {
        self.custom_flags(libc::O_DIRECT);

        if is_sync {
            self.custom_flags(libc::O_SYNC);
        }

        let tokio_file = tokio::fs::OpenOptions::from(self).open(path).await?;

        Ok(crate::fs::File { inner: tokio_file })
    }
}
