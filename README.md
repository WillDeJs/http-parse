# HTTP Parse # 

## Educational Purposes Only ## 

This is some HTTP parsing library written in the most naive way possible just as a learning project.

A hand-written and naive implementation of a parser for the HTTP Protocol.

 This libary does not aim to be complete but instead to be an educational project.
 ## Example Parsing a Simple Response: ##
 ```Rust
 use std::io::Cursor;
 use http_parse::{HttpHeader, HttpResponseBuilder, HttpMethod, HttpVersion, HttpParser};
 let request = "GET / HTTP/1.1\r
 Host: developer.mozilla.org\r
 Accept-Language: fr\r\n";

let mut reader = Cursor::new(request.as_bytes());
let mut parser = HttpParser::from_reader(&mut reader);
let request = parser.request().unwrap();

assert_eq!(request.version(), HttpVersion::Http11);
assert_eq!(request.method(), HttpMethod::Get);

let my_header = HttpHeader::new("host", "developer.mozilla.org");
assert_eq!(request.header("host"), Some(&my_header));
 ```
 ## Example a simple Client: ##
 ```Rust
use http_parse::{HttpMethod, HttpParser, HttpRequest, HttpUrl, StatusCode, H_HOST, H_USER_AGENT};
use std::io::Write;
use std::net::TcpStream;

pub struct Client;
impl Client {
    fn download(url: &HttpUrl) -> Result<(), String> {
        let mut client = TcpStream::connect(url.address()).map_err(|e| e.to_string())?;
        let file = url.file().unwrap_or(url.path());
        let request = HttpRequest::builder()
            .method(HttpMethod::Get)
            .path(url.path())
            .header(H_USER_AGENT, "Mozilla/5.0 (WD TEST)")
            .header(H_HOST, url.host())
            .build();
        client
            .write_all(&request.into_bytes())
            .map_err(|e| e.to_string())?;
        let response = HttpParser::from_reader(&mut client)
            .response()
            .map_err(|e| e.to_string())?;

        if response.status_code() != StatusCode::OK {
            return Err(format!(
                "Unexpected status code `{}` from server",
                response.status_code()
            ));
        }
        let mut out_file = std::fs::File::create(file).map_err(|e| e.to_string())?;
        out_file
            .write_all(response.data())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
fn main() -> Result<(), String> {
    let url = HttpUrl::try_from("192.168.1.8:8080/Video.mp4").unwrap();
    Client::download(&url)?;
    Ok(())
}

 ```