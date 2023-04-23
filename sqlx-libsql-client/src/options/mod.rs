mod connect;
mod parse;

/// Options and flags which can be used to configure a libSQL connection.
/// # Example
///
/// ```rust,no_run
/// # use sqlx_core::connection::ConnectOptions;
/// # use sqlx_core::error::Error;
/// # use sqlx_sqlite::{LibsqlClientConnectOptions, LibsqlClientJournalMode};
/// use std::str::FromStr;
///
/// # fn main() {
/// # #[cfg(feature = "_rt-async-std")]
/// # sqlx::__rt::test_block_on(async move {
/// let conn = LibsqlClientConnectOptions::from_str("libsql://localhost:8080")?
///     .connect().await?;
/// # Result::<(), Error>::Ok(())
/// # }).unwrap();
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct LibsqlClientConnectOptions {
    pub(crate) url: url::Url,
    pub(crate) auth_token: Option<String>,
}

impl Default for LibsqlClientConnectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl LibsqlClientConnectOptions {
    /// Construct `Self` with default options.
    ///
    /// See the source of this method for the current defaults.
    pub fn new() -> Self {
        Self {
            url: url::Url::parse("http://localhost:8080").unwrap(),
            auth_token: None,
        }
    }

    /// Sets the url to connect to.
    pub fn url(mut self, url: impl Into<url::Url>) -> Self {
        self.url = url.into();
        self
    }

    /// Sets the auth token to use for authentication.
    pub fn auth_token(mut self, auth_token: impl Into<String>) -> Self {
        self.auth_token = Some(auth_token.into());
        self
    }
}
