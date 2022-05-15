use crate::Auth;
use reqwest::{
    header::{self, HeaderMap},
    tls, Client, ClientBuilder,
};
use std::time::Duration;
use zeroize::Zeroize;

pub(crate) fn http_client_builder() -> ClientBuilder {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .https_only(true)
        .user_agent(
            // FIXME: get rid of hardcoded user agent
            concat!(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 12_3_1)",
                " AppleWebKit/537.36 (KHTML, like Gecko)",
                " Chrome/101.0.4951.67 Safari/537.36"
            ),
        )
        .min_tls_version(tls::Version::TLS_1_2)
}

pub(crate) fn default_headers(auth_method: &Auth) -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    set_auth_header(&mut headers, auth_method);
    headers
}

#[inline]
fn set_auth_header(headers: &mut HeaderMap, auth_method: &Auth) {
    let mut auth = match auth_method {
        Auth::BearerToken(token) => {
            format!("Bearer {}", token) // Token instead of Bearer?
        }
    };
    headers.insert(header::AUTHORIZATION, auth.parse().unwrap());
    auth.zeroize();
}
