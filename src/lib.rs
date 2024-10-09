use std::fmt::Display;
use std::io:: Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}
impl PartialEq for HttpHeader {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase()) && self.value == other.value
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
    pub fn headers(&self) -> Vec<&HttpHeader> {
        self.headers.iter().collect()
    }
    pub fn header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers
            .iter()
            .find(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
    }
    pub fn put_header(&mut self, name: &str, value: &str) {
        if let Some(index) = self
            .headers
            .iter()
            .position(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
        {
            self.headers[index].value = value.to_string();
        } else {
            self.headers.push(HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
    }
    pub fn method(&self) -> HttpMethod {
        self.method
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // first line, version + status code  + msg
        bytes.extend_from_slice(
            &format!(
                "{} {} {}\r\n",
                self.method(),
                self.url.inner,
                self.version()
            )
            .into_bytes(),
        );
        // next all headers
        for header in self.headers() {
            bytes.extend_from_slice(&format!("{}\r\n", header).into_bytes());
        }

        // next the body
        if !self.body.is_empty() {
            bytes.push(b'\r');
            bytes.push(b'\n');
            bytes.extend_from_slice(&self.body);
        }
        bytes
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
        if let Some(index) = self
            .headers
            .iter()
            .position(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
        {
            self.headers[index].value = value.to_string();
        } else {
            self.headers.push(HttpHeader {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
    }

    pub fn headers(&self) -> Vec<&HttpHeader> {
        self.headers.iter().collect()
    }
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // first line, version + status code  + msg
        bytes.extend_from_slice(
            &format!(
                "{} {} {}\r\n",
                self.version(),
                self.status_code(),
                self.status_msg()
            )
            .into_bytes(),
        );
        // next all headers
        for header in self.headers() {
            bytes.extend_from_slice(&format!("{}\r\n", header).into_bytes());
        }

        // next the body
        if !self.body.is_empty() {
            bytes.push(b'\r');
            bytes.push(b'\n');
            bytes.extend_from_slice(&self.body);
        }
        bytes
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
    pub fn new(buffer: &[u8]) -> Self {
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
        match version {
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

    fn parse_headers(&mut self) -> std::io::Result<Vec<HttpHeader>> {
        let mut headers = Vec::new();
           
        let mut name_buffer = Vec::new();
        while ! self.inner.is_line_end() {
            let mut value = String::new();
            match self.inner.read_until(b':', &mut name_buffer) {
                Ok(n) => {
                    if n == 0 {
                        // no data = done reading
                        return Ok(headers);
                    }
                    let name = String::from_utf8_lossy(&name_buffer[0..n-1]).to_string();
                    self.inner.skip_white_space(); // spaces may occur before header value from standard
                    self.inner.read_sentence(&mut value);
                    self.inner.next_line(); // read line ending left over from sentence
                    headers.push(HttpHeader { name, value });
                    
                }
                Err(error) => {
                    return Err(error);
                }
            }
            name_buffer.clear();
        }
        self.inner.next_line();
        Ok(headers)
    }

    fn parse_body(&mut self) -> Result<Vec<u8>, Error> {
        let mut buff = Vec::new();
        match self.inner.read_to_end(&mut buff) {
            Ok(_n) => Ok(buff),
            Err(e) => Err(e),
        }
    }
    pub fn parse_response(&mut self) -> Result<HttpResponse, Error> {

        let mut version = String::new();
        self.inner.read_word(&mut version);
        self.inner.skip_white_space();
        let version = Self::parse_version(version.as_bytes())?;
        
        let mut status_code = String::new();
        self.inner.read_word(&mut status_code);
        self.inner.skip_white_space();
        let status_code = Self::parse_status_code(status_code.as_bytes())?;

        let mut status_msg = String::new();
        self.inner.read_sentence(&mut status_msg);
        self.inner.next_line();
        let headers = self.parse_headers()?;
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
        let mut method = String::new();
        self.inner.read_word(&mut method);
        let method = Self::parse_method(method.as_bytes())?;
        self.inner.skip_white_space();

        let mut url = String::new();
        self.inner.read_word(&mut url);
        self.inner.skip_white_space();

        let mut version = String::new();
        self.inner.read_word(&mut version);
        let version = Self::parse_version(version.as_bytes())?;
        self.inner.next_line();

        let headers = self.parse_headers()?;
        let body = self.parse_body()?;
        Ok(HttpRequest {
            method,
            url: Url{ inner: url},
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
    pub(crate) fn new(buffer: &[u8]) -> Self {
        Self {
            inner: buffer.to_vec(),
            index: 0,
        }
    }

    pub fn skip_white_space(&mut self) {
        while self.index < self.inner.len() {
            if !(self.inner[self.index] as char).is_whitespace() {
                break;
            }
            self.index += 1;
        }
    }
    pub(crate) fn read_word(&mut self, buffer: &mut String) -> usize {
        let mut read = 0;
        while self.index < self.inner.len() {
            let current_char = self.inner[self.index] as char;
            if !current_char.is_whitespace() {
                buffer.push(self.inner[self.index] as char);
                read += 1;
            } else {
                break;
            }
            self.index += 1;
        }
        read
    }
    pub (crate) fn is_line_end(&self) -> bool {
        if self.index < self.inner.len() {
            let current_char = self.inner[self.index];
            current_char ==  b'\r' || current_char == b'\n' 
        } else {
            true
        }
        
    }
    pub(crate) fn next_line(&mut self) -> bool {
        while self.index < self.inner.len() {
            let current_char = self.inner[self.index];
            if current_char == b'\n'{
                self.index +=1;
                return true;
            }
            self.index += 1;
        } 
        false
    }
    pub(crate) fn read_sentence(&mut self, buffer: &mut String) -> usize {
        let mut read = 0;
        while self.index < self.inner.len() {
            if self.inner[self.index] == b'\r' || self.inner[self.index] == b'\n' {
                break;
            } else { 
                buffer.push(self.inner[self.index] as char);
                read += 1;
            }
            self.index += 1;
        }
        read
    }
    pub(crate) fn read_until(&mut self, byte: u8, buffer: &mut Vec<u8>) -> std::io::Result<usize> {
        let mut count = 0;
        while self.index < self.inner.len() {
            let current_char = self.inner[self.index];
            buffer.push(current_char);
            count+=1;
            self.index += 1;
            if current_char == byte {
                break;
            }
        }
          Ok(count)
    }

    pub (crate) fn read_to_end(&mut self, buffer: &mut Vec<u8>) -> std::io::Result<usize> {
        let mut count = 0;
        let start = self.index;
        let end = self.inner.len();
        if self.index < self.inner.len() {
            buffer.extend_from_slice(&self.inner[start..end]);
            count = end - start;
            self.index = end;
        }
        Ok(count)
    }
}

