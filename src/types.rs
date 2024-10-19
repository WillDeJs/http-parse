use std::{fmt::Display, str::FromStr};

use crate::{StatusCode, H_CONTENT_LENGTH, H_TRANSFER_ENCODING};

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

#[derive(Debug, Clone)]
pub struct HttpHeader {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl HttpHeader {
    pub fn new<T, V>(name: T, value: V) -> Self
    where
        V: Display,
        T: Display,
    {
        HttpHeader {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn value<T: FromStr>(&self) -> Result<T, T::Err> {
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
    pub(crate) chunks: Vec<(usize, usize)>,
    pub(crate) chunked: bool,
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
            chunks: Vec::new(),
            chunked: false,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
        if self.chunked {
            self.chunks.push((self.body.len(), data.len()));
            self.put_header(H_TRANSFER_ENCODING, "chunked");
        } else {
            self.put_header(H_CONTENT_LENGTH, self.body.len());
        }
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
    pub fn header<T>(&self, name: T) -> Option<&HttpHeader>
    where
        T: AsRef<str>,
    {
        self.headers.iter().find(|header| {
            header
                .name
                .to_lowercase()
                .eq(&name.as_ref().to_lowercase().to_string())
        })
    }
    pub fn put_header<T>(&mut self, name: &str, value: T)
    where
        T: Display,
    {
        if let Some(index) = self
            .headers
            .iter()
            .position(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
        {
            self.headers[index].value = format!("{}", value);
        } else {
            self.headers.push(HttpHeader {
                name: name.to_string(),
                value: format!("{}", value),
            });
        }
    }
    pub fn remove_header(&mut self, name: &str) {
        self.headers
            .retain(|header| header.name.to_lowercase().ne(&name.to_lowercase()));
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
        bytes.push(b'\r');
        bytes.push(b'\n');

        // next the body
        if !self.body.is_empty() {
            if self.chunked {
                for (start, end) in &self.chunks {
                    let count = end - start;
                    bytes.extend_from_slice(format!("{:X}\r\n", count).as_bytes());
                    bytes.extend_from_slice(&self.body[*start..*end]);

                    bytes.push(b'\r');
                    bytes.push(b'\n');
                }
            } else {
                bytes.extend_from_slice(&self.body);
            }
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
        if self.chunked {
            for (start, end) in &self.chunks {
                let count = end - start;
                write!(f, "{:X}\r\n", count)?;
                write!(
                    f,
                    "{}\r\n",
                    String::from_utf8_lossy(&self.body[*start..*end])
                )?;

                if count == 0 {
                    write!(f, "\r\n\r\n")?;
                }
            }
            Ok(())
        } else {
            write!(f, "{}", String::from_utf8_lossy(&self.body))
        }
    }
}

#[derive(PartialEq)]
pub struct HttpResponse {
    pub(crate) version: HttpVersion,
    pub(crate) status_code: usize,
    pub(crate) status_msg: String,
    pub(crate) headers: Vec<HttpHeader>,
    pub(crate) body: Vec<u8>,
    pub(crate) chunks: Vec<(usize, usize)>,
    pub(crate) chunked: bool,
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
            chunks: Vec::new(),
            chunked: false,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
        if self.chunked {
            self.chunks.push((self.body.len(), data.len()));
            self.put_header(H_TRANSFER_ENCODING, "chunked");
        } else {
            self.put_header(H_CONTENT_LENGTH, self.body.len());
        }
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
    pub fn put_header<T>(&mut self, name: &str, value: T)
    where
        T: Display,
    {
        if let Some(index) = self
            .headers
            .iter()
            .position(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
        {
            self.headers[index].value = format!("{}", value);
        } else {
            self.headers.push(HttpHeader {
                name: name.to_string(),
                value: format!("{}", value),
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
        bytes.push(b'\r');
        bytes.push(b'\n');

        // next the body
        if !self.body.is_empty() {
            if self.chunked {
                for (start, end) in &self.chunks {
                    let count = end - start;
                    bytes.extend_from_slice(format!("{:X}\r\n", count).as_bytes());
                    bytes.extend_from_slice(&self.body[*start..*end]);

                    bytes.push(b'\r');
                    bytes.push(b'\n');
                }
            } else {
                bytes.extend_from_slice(&self.body);
            }
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

        if self.chunked {
            for (start, end) in &self.chunks {
                let count = end - start;
                write!(f, "{:X}\r\n", count)?;
                write!(
                    f,
                    "{}\r\n",
                    String::from_utf8_lossy(&self.body[*start..*end])
                )?;

                if count == 0 {
                    write!(f, "\r\n")?;
                }
            }
            Ok(())
        } else {
            write!(f, "{}", String::from_utf8_lossy(&self.body))
        }
    }
}

pub struct HttpResponseBuilder {
    version: Option<HttpVersion>,
    status_code: Option<StatusCode>,
    headers: Option<Vec<HttpHeader>>,
    data: Option<Vec<u8>>,
    chunks: Option<Vec<(usize, usize)>>,
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self {
            version: Some(HttpVersion::Http1),
            status_code: Some(StatusCode::OK),
            headers: None,
            data: None,
            chunks: None,
        }
    }
    pub fn status(mut self, code: StatusCode) -> Self {
        self.status_code = Some(code);
        self
    }
    pub fn header<T>(mut self, name: &str, value: T) -> Self
    where
        T: Display,
    {
        let headers = self.headers.get_or_insert(Vec::new());
        headers.push(HttpHeader::new(name, value));
        self
    }
    pub fn body(mut self, new_data: &[u8]) -> Self {
        let data = self.data.get_or_insert(Vec::new());
        data.extend_from_slice(new_data);
        self
    }
    pub fn build(self) -> HttpResponse {
        let version = self.version.unwrap();
        let status = self.status_code.unwrap();
        let body = self.data.unwrap_or_default();
        let headers = self.headers.unwrap_or_default();
        HttpResponse {
            version,
            status_code: status.0,
            status_msg: status.1.to_string(),
            body,
            headers,
            chunks: Vec::new(),
            chunked: false,
        }
    }
}

pub struct HttpRequestBuilder {
    method: Option<HttpMethod>,
    url: Option<String>,
    version: Option<HttpVersion>,
    headers: Option<Vec<HttpHeader>>,
    data: Option<Vec<u8>>,
    chunks: Option<Vec<(usize, usize)>>,
}

impl Default for HttpRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        Self {
            method: Some(HttpMethod::Get),
            url: Some("/".to_string()),
            version: Some(HttpVersion::Http1),
            headers: None,
            data: None,
            chunks: None,
        }
    }
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }
    pub fn header<T>(mut self, name: &str, value: T) -> Self
    where
        T: std::fmt::Display,
    {
        let headers = self.headers.get_or_insert(Vec::new());
        headers.push(HttpHeader::new(name, value));
        self
    }
    pub fn body(mut self, new_data: &[u8]) -> Self {
        let data = self.data.get_or_insert(Vec::new());
        data.extend_from_slice(new_data);
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
    pub fn build(self) -> HttpRequest {
        let version = self.version.unwrap();
        let method = self.method.unwrap();
        let body = self.data.unwrap_or_default();
        let headers = self.headers.unwrap_or_default();
        let url = self.url.unwrap();
        HttpRequest {
            version,
            body,
            headers,
            chunks: Vec::new(),
            chunked: false,
            method,
            url: Url {
                inner: url.to_string(),
            },
        }
    }
}
