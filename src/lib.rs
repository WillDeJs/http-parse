use std::fmt::Display;
use std::io::{BufRead, Error, Read};

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
pub enum MsgType {
    Request,
    Response,
}

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Trace,
    Head,
    Options,
    Patch,
    Delete,
    Connect,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Trace => write!(f, "TRACE"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Connect => write!(f, "CONNECT"),
        }
    }
}

#[derive( Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}
impl PartialEq for HttpHeader {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase()) && self.value ==other.value
    }
}
impl Display for HttpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HttpVersion {
    Http1,
    Http2,
    Http3,
}
impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}
#[derive(PartialEq)]
pub struct Url {
    inner: String,
}
#[derive(PartialEq)]
pub struct HttpRequest {
    method: HttpMethod,
    url: Url,
    version: HttpVersion,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

impl HttpRequest {
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        let mut parser = HttpParser::new(buffer);
        parser.parse_request()
    }
    pub fn new() -> Self {
        Self {
            version: HttpVersion::Http1,
            headers: Vec::new(),
            body: Vec::new(),
            method: HttpMethod::Get,
            url: Url {
                inner: "\\".to_string(),
            },
        }
    }
    
    pub fn with_method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }
    pub fn with_version(mut self, version: HttpVersion) -> Self {
        self.version = version;
        self
    }
    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.body
    }
    pub fn version(&self) -> HttpVersion {
        self.version
    }
    pub fn header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers
            .iter()
            .find(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
    }
    pub fn put_header(&mut self, name: &str, value: &str) {
        if let Some(index) = self.headers.iter().position(|header| header.name.to_lowercase().eq(&name.to_lowercase())) {
            self.headers[index].value=value.to_string();
        } else {
            self.headers.push( HttpHeader {
                name: name.to_string(),
                value: value.to_string()
            });
        }
    }
    pub fn method(&self) -> HttpMethod {
        self.method
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.url.inner, self.version)?;
        self.headers.iter().for_each(|header| {
            let _ = write!(f, "{}\r\n", header);
        });
        write!(f, "\r\n")?;
        write!(f, "{}", String::from_utf8_lossy(&self.body))
    }
}

#[derive(PartialEq)]
pub struct HttpResponse {
    version: HttpVersion,
    status_code: usize,
    status_msg: String,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, Error> {
        let mut parser = HttpParser::new(buffer);
        parser.parse_response()
    }
    pub fn new() -> HttpResponse {
        Self {
            version: HttpVersion::Http1,
            status_code: 200,
            status_msg: "Ok".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn with_status_code(mut self, code: usize) -> Self {
        self.status_code = code;
        self
    }
    pub fn with_version(&mut self, version: HttpVersion) {
        self.version = version;
    }
    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.body
    }
    pub fn version(&self) -> HttpVersion {
        self.version
    }
    pub fn status_code(&self) -> usize {
        self.status_code
    }
    pub fn status_msg(&self) -> String {
        self.status_msg.clone()
    }
    pub fn header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers
            .iter()
            .find(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
    }
    pub fn put_header(&mut self, name: &str, value: &str) {
        if let Some(index) = self.headers.iter().position(|header| header.name.to_lowercase().eq(&name.to_lowercase())) {
            self.headers[index].value=value.to_string();
        } else {
            self.headers.push( HttpHeader {
                name: name.to_string(),
                value: value.to_string()
            });
        }
    }
    
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_msg
        )?;
        self.headers.iter().for_each(|header| {
            let _ = write!(f, "{}\r\n", header);
        });
        write!(f, "\r\n")?;
        write!(f, "{}", String::from_utf8_lossy(&self.body))
    }
}
pub struct HttpParser {
    inner: ByteBuffer,
}

impl HttpParser {
    pub fn new<'a>(buffer: &'a [u8]) -> Self {
        Self {
            inner: ByteBuffer::new(buffer),
        }
    }

    fn parse_method(method: &[u8]) -> Result<HttpMethod, Error> {
        match method.trim_ascii() {
            b"GET" => Ok(HttpMethod::Get),
            b"POST" => Ok(HttpMethod::Post),
            b"PUT" => Ok(HttpMethod::Put),
            b"HEAD" => Ok(HttpMethod::Head),
            b"OPTIONS" => Ok(HttpMethod::Options),
            b"DELETE" => Ok(HttpMethod::Delete),
            b"TRACE" => Ok(HttpMethod::Trace),

            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid HTTP method in buffer `{}`.",
                    String::from_utf8_lossy(method)
                ),
            )),
        }
    }

    fn parse_version(version: &[u8]) -> Result<HttpVersion, Error> {
        match version.trim_ascii() {
            b"HTTP/1.1" => Ok(HttpVersion::Http1),
            b"HTTP/2" => Ok(HttpVersion::Http2),
            b"HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid HTTP version in buffer `{}`.",
                    String::from_utf8_lossy(version)
                ),
            )),
        }
    }

    fn parse_url(url: &[u8]) -> Url {
        let url_string = String::from_utf8_lossy(url);
        Url {
            inner: url_string.to_string(),
        }
    }
    fn parse_status_code(status_code: &[u8]) -> Result<usize, Error> {
        let code_string = String::from_utf8_lossy(status_code);
        match code_string.trim().parse::<usize>() {
            Ok(value) => Ok(value),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid HTTP status code  in buffer `{}`.",
                    String::from_utf8_lossy(status_code)
                ),
            )),
        }
    }

    fn parse_headers(&mut self) -> Vec<HttpHeader> {
        let mut headers = Vec::new();
        let mut line = String::new();
        while let Ok(_) = self.inner.read_line(&mut line) {
            if line.trim().is_empty() {
                break;
            } else {
                if let Some(index) = line.find(':') {
                    if index < line.len() {
                        let name = line[0..index].to_string().trim().to_string();
                        let value = line[index + 1..line.len() - 1].trim_ascii_start().to_string();
                        headers.push(HttpHeader { name, value });
                    }
                }
                line.clear();
            }
        }
        headers
    }
    fn parse_body(&mut self) -> Result<Vec<u8>, Error> {
        let mut buff = Vec::new();
        match self.inner.read_to_end(&mut buff) {
            Ok(_n) => Ok(buff),
            Err(e) => Err(e),
        }
    }
    pub fn parse_response(&mut self) -> Result<HttpResponse, Error> {
        let mut buffer = Vec::new();
        self.inner.read_until(b' ', &mut buffer)?;
        let version = Self::parse_version(&buffer)?;
        buffer.clear();
        self.inner.read_until(b' ', &mut buffer)?;
        let status_code = Self::parse_status_code(&buffer)?;

        buffer.clear();
        self.inner.read_until(b'\n', &mut buffer)?;
        let status_msg = String::from_utf8_lossy(&buffer).trim().to_string();
        let headers = self.parse_headers();
        let body = self.parse_body()?;
        Ok(HttpResponse {
            version,
            status_code,
            status_msg,
            headers,
            body,
        })
    }
    pub fn parse_request(&mut self) -> Result<HttpRequest, Error> {
        let mut buffer = Vec::new();
        self.inner.read_until(b' ', &mut buffer)?;
        let method = Self::parse_method(&buffer)?;
        buffer.clear();
        self.inner.read_until(b' ', &mut buffer)?;
        let url = Self::parse_url(&buffer);
        buffer.clear();
        self.inner.read_until(b'\n', &mut buffer)?;
        let version = Self::parse_version(&buffer)?;

        let headers = self.parse_headers();
        let body = self.parse_body()?;
        Ok(HttpRequest {
            method,
            url,
            version,
            headers,
            body,
        })
    }
}
pub struct ByteBuffer {
    inner: Vec<u8>,
    index: usize,
}

impl ByteBuffer {
    pub fn new(buffer: &[u8]) -> Self {
        Self {
            inner: buffer.to_vec(),
            index: 0,
        }
    }
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Read for ByteBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        let inner_len = self.inner.len();
        let start = self.index;

        if start + buf_len <= inner_len {
            buf.copy_from_slice(&self.inner[start..start + buf_len]);
            self.index += buf_len;
            Ok(buf_len)
        } else if start < inner_len {
            let leftover_count = inner_len - self.index;
            buf[0..leftover_count].copy_from_slice(&self.inner[start..start + leftover_count]);
            self.index += leftover_count;
            Ok(leftover_count)
        } else {
            Ok(0) // done reading
        }
    }
}
impl BufRead for ByteBuffer {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.inner.len() > self.index {
            Ok(&self.inner[self.index..])
        } else {
            Ok(&[])
        }
    }

    fn consume(&mut self, amt: usize) {
        if self.index + amt <= self.inner.len() {
            self.index += amt;
        }
    }
}
