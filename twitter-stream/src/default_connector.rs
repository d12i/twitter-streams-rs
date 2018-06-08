
cfg_if! {
    if #[cfg(feature = "tls")] {
        extern crate hyper_tls;
        extern crate native_tls;

        pub use self::native_tls::Error;

        use hyper::client::HttpConnector;
        use self::hyper_tls::HttpsConnector;

        pub fn new() -> Result<HttpsConnector<HttpConnector>, Error> {
            HttpsConnector::new(1)
        }
    } else if #[cfg(feature = "tls-rustls")] {
        extern crate hyper_rustls;

        pub use util::Never as Error;

        use self::hyper_rustls::HttpsConnector;

        pub fn new(h: &::tokio_core::reactor::Handle) -> Result<HttpsConnector, Error> {
            Ok(HttpsConnector::new(1, h))
        }
    } else if #[cfg(feature = "tls-openssl")] {
        extern crate hyper_openssl;

        pub use self::hyper_openssl::openssl::error::ErrorStack as Error;

        use hyper::client::HttpConnector;
        use self::hyper_openssl::HttpsConnector;

        pub fn new() -> Result<HttpsConnector<HttpConnector>, Error> {
            HttpsConnector::new(1)
        }
    } else {
        pub use util::Never as Error;

        use hyper::client::HttpConnector;

        #[cold]
        pub fn new() -> Result<HttpConnector, Error> {
            Ok(HttpConnector::new(1))
        }
    }
}
