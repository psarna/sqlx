pub use async_std::{
    self, future::timeout, io::prelude::ReadExt as AsyncReadExt,
    io::prelude::WriteExt as AsyncWriteExt, io::Read as AsyncRead, io::Write as AsyncWrite,
    sync::Mutex as AsyncMutex,
};
#[cfg(feature = "fs_and_spawn")]
pub use async_std::{fs, net::TcpStream, task::sleep, task::spawn, task::yield_now};

#[cfg(unix)]
pub use async_std::os::unix::net::UnixStream;

#[cfg(all(feature = "_tls-native-tls", not(feature = "_tls-rustls")))]
pub use async_native_tls::{TlsConnector, TlsStream};

#[cfg(all(feature = "_tls-rustls", not(feature = "_tls-native-tls")))]
pub use futures_rustls::{client::TlsStream, TlsConnector};

pub use async_std::task::{block_on, block_on as test_block_on};

pub fn enter_runtime<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // no-op for async-std
    f()
}
