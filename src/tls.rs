//!secure connection over TLS

use std::prelude::v1::*;
use crate::error::Error as HttpError;
use std::{
    untrusted::fs::File,
    io::{self, BufReader},
    path::Path,
};

#[cfg(feature = "native-tls")]
use std::io::prelude::*;

#[cfg(feature = "rust-tls")]
use crate::error::ParseErr;

//#[cfg(not(any(feature = "native-tls", feature = "rust-tls")))]
//compile_error!("one of the `native-tls` or `rust-tls` features must be enabled");

///wrapper around TLS Stream,
///depends on selected TLS library
pub struct Conn<S: io::Read + io::Write> {
    #[cfg(feature = "native-tls")]
    stream: native_tls::TlsStream<S>,

    #[cfg(feature = "rust-tls")]
    stream: rustls::StreamOwned<rustls::ClientSession, S>,
}

impl<S: io::Read + io::Write> io::Read for Conn<S> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let len = self.stream.read(buf);

        #[cfg(feature = "rust-tls")]
        {
            // TODO: this api returns ConnectionAborted with a "..CloseNotify.." string.
            // TODO: we should work out if self.stream.sess exposes enough information
            // TODO: to not read in this situation, and return EOF directly.
            // TODO: c.f. the checks in the implementation. connection_at_eof() doesn't
            // TODO: seem to be exposed. The implementation:
            // TODO: https://github.com/ctz/rustls/blob/f93c325ce58f2f1e02f09bcae6c48ad3f7bde542/src/session.rs#L789-L792
            if let Err(ref e) = len {
                if io::ErrorKind::ConnectionAborted == e.kind() {
                    return Ok(0);
                }
            }
        }

        len
    }
}

impl<S: io::Read + io::Write> io::Write for Conn<S> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.stream.write(buf)
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.stream.flush()
    }
}

///client configuration
pub struct Config {
    #[cfg(feature = "native-tls")]
    extra_root_certs: Vec<native_tls::Certificate>,
    #[cfg(feature = "rust-tls")]
    client_config: std::sync::Arc<rustls::ClientConfig>,
}

impl Default for Config {
    #[cfg(feature = "native-tls")]
    fn default() -> Self {
        Config {
            extra_root_certs: vec![],
        }
    }

    #[cfg(feature = "rust-tls")]
    fn default() -> Self {
        let mut config = rustls::ClientConfig::new();
        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        Config {
            client_config: std::sync::Arc::new(config),
        }
    }
}

impl Config {
    #[cfg(feature = "native-tls")]
    pub fn add_root_cert_file_pem(&mut self, file_path: &Path) -> Result<&mut Self, HttpError> {
        let f = File::open(file_path)?;
        let f = BufReader::new(f);
        let mut pem_crt = vec![];
        for line in f.lines() {
            let line = line?;
            let is_end_cert = line.contains("-----END");
            pem_crt.append(&mut line.into_bytes());
            pem_crt.push(b'\n');
            if is_end_cert {
                let crt = native_tls::Certificate::from_pem(&pem_crt)?;
                self.extra_root_certs.push(crt);
                pem_crt.clear();
            }
        }
        Ok(self)
    }

    #[cfg(feature = "native-tls")]
    pub fn connect<H, S>(&self, hostname: H, stream: S) -> Result<Conn<S>, HttpError>
    where
        H: AsRef<str>,
        S: io::Read + io::Write,
    {
        let mut connector_builder = native_tls::TlsConnector::builder();
        for crt in self.extra_root_certs.iter() {
            connector_builder.add_root_certificate((*crt).clone());
        }
        let connector = connector_builder.build()?;
        let stream = connector.connect(hostname.as_ref(), stream)?;

        Ok(Conn { stream })
    }

    #[cfg(feature = "rust-tls")]
    pub fn add_root_cert_file_pem(&mut self, file_path: &Path) -> Result<&mut Self, HttpError> {
        let f = File::open(file_path)?;
        let mut f = BufReader::new(f);
        let config = std::sync::Arc::make_mut(&mut self.client_config);
        let _ = config
            .root_store
            .add_pem_file(&mut f)
            .map_err(|_| HttpError::from(ParseErr::Invalid))?;
        Ok(self)
    }

    #[cfg(feature = "rust-tls")]
    pub fn connect<H, S>(&self, hostname: H, stream: S) -> Result<Conn<S>, HttpError>
    where
        H: AsRef<str>,
        S: io::Read + io::Write,
    {
        use rustls::{ClientSession, StreamOwned};

        let session = ClientSession::new(
            &self.client_config,
            webpki::DNSNameRef::try_from_ascii_str(hostname.as_ref())
                .map_err(|_| HttpError::Tls)?,
        );
        let stream = StreamOwned::new(session, stream);

        Ok(Conn { stream })
    }
}
