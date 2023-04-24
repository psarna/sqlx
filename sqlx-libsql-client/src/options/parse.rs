use crate::error::Error;
use crate::LibsqlClientConnectOptions;
use percent_encoding::percent_decode_str;
use std::str::FromStr;
impl LibsqlClientConnectOptions {
    pub(crate) fn from_url_and_token(url: &str, params: Option<&str>) -> Result<Self, Error> {
        let options = Self {
            url: url::Url::parse(url).map_err(|e| Error::Protocol(e.to_string()))?,
            auth_token: params.and_then(|params| {
                params
                    .split('&')
                    .find(|param| param.starts_with("auth_token="))
                    .map(|param| param.split('=').nth(1).unwrap_or_default())
                    .map(|token| percent_decode_str(token).decode_utf8_lossy().into_owned())
            }),
        };
        Ok(options)
    }
}

impl FromStr for LibsqlClientConnectOptions {
    type Err = Error;

    fn from_str(mut url: &str) -> Result<Self, Self::Err> {
        // remove scheme from the URL
        url = url
            .trim_start_matches("libsql://")
            .trim_start_matches("libsql:");

        let mut database_and_params = url.splitn(2, '?');

        let database = database_and_params.next().unwrap_or_default();
        let params = database_and_params.next();

        Self::from_url_and_token(database, params)
    }
}
