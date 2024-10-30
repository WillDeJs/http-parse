//! A hand-written and naive implementation of a parser for the HTTP Protocol.
//!
//! This libary does not aim to be complete but instead to be an educational project.
//! # Example parsing a simple resonse:
//! ```no_run
//! use http_parse::{HttpHeader, HttpResponseBuilder, HttpMethod, HttpVersion, HttpParser, ByteBuffer};
//! let request = "GET / HTTP/1.1\r
//! Host: developer.mozilla.org\r
//! Accept-Language: fr\r\n";
//!
//!     let mut reader = ByteBuffer::new(request.as_bytes());
//!     let mut parser = HttpParser::from_reader(&mut reader);
//!     let request = parser.request().unwrap();
//!
//!     assert_eq!(request.version(), HttpVersion::Http11);
//!     assert_eq!(request.method(), HttpMethod::Get);
//!     let my_header = HttpHeader::new("host", "developer.mozilla.org");
//!     assert_eq!(request.header("host"), Some(&my_header));
//! ```
//! # Example a simple Client:
//! ```no_run
//! use http_parse::{
//!     HttpParser, HttpRequestBuilder, H_ACCEPT_RANGES, H_CONNECTION, H_HOST, H_USER_AGENT,
//! };
//! use std::io::Write;
//! use std::net::TcpStream;
//!
//! pub struct Client;
//! impl Client {
//!     pub fn download(host: &str, req: &str) -> std::io::Result<()> {
//!         // Connect to server
//!         let mut client = TcpStream::connect(host)?;
//!
//!         // Prepare a request to send to the server
//!         let request = HttpRequestBuilder::new()
//!             .header(H_ACCEPT_RANGES, "bytes")
//!             .header(H_CONNECTION, "close")
//!             .header(H_HOST, "192.168.1.8")
//!             .header(H_USER_AGENT, "Mozzila/5.0 (WD test)")
//!             .build();
//!
//!         // Send request to the server
//!         client.write_all(&request.into_bytes())?;
//!
//!         // Prepare to parse the server response
//!         let mut parser = HttpParser::from_reader(&mut client);
//!         let response = parser.response()?;
//!
//!         // Write teh server's response to the local file.
//!         let mut file = std::fs::File::create(&format!(".{}", req))?;
//!         file.write_all(response.data())?;
//!         Ok(())
//!     }
//! }
//! fn main() -> std::io::Result<()> {
//!     Client::download("192.168.1.8:8080", "/Video.mp4")?;
//!     Ok(())
//! }
//! ```
//!
#[allow(unused)]
mod definitions;
#[allow(unused)]
mod parser;
#[allow(unused)]
mod types;

pub use definitions::*;
pub use parser::ByteBuffer;
pub use parser::HttpParser;

pub use types::HttpHeader;
pub use types::HttpMethod;
pub use types::HttpRequest;
pub use types::HttpResponse;
pub use types::HttpUrl;
pub use types::HttpVersion;

pub use types::HttpParseError;
pub use types::HttpRequestBuilder;
pub use types::HttpResponseBuilder;
pub use types::HttpUrlBuilder;
