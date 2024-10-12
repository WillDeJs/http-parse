use std::{fmt::Display, str::FromStr};


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

impl HttpHeader {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn value<T : FromStr>(&self) -> Result<T, T::Err> {
        self.value.parse::<T>()
    }
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
    pub(crate) inner: String,
}
#[derive(PartialEq)]
pub struct HttpRequest {
    pub(crate) method: HttpMethod,
    pub(crate) url: Url,
    pub(crate) version: HttpVersion,
    pub(crate) headers: Vec<HttpHeader>,
    pub(crate) body: Vec<u8>,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequest {
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
    pub(crate) version: HttpVersion,
    pub(crate) status_code: usize,
    pub(crate) status_msg: String,
    pub(crate) headers: Vec<HttpHeader>,
    pub(crate) body: Vec<u8>,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponse {
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

