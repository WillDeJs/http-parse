use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{
    StatusCode, DEFAULT_HTTPS_PORT, DEFAULT_HTTP_PORT, H_CONTENT_LENGTH, H_TRANSFER_ENCODING,
};

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

/// A Simple HTTP Header implementation.
#[derive(Debug, Clone)]
pub struct HttpHeader {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl HttpHeader {
    /// Create a new HTTP Header parting from a header name and value.
    /// #Arguments
    /// `name` the name of the header to be inserted
    /// `value` the value being added for this header.
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
    /// The name for a header
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the value for this header, parse it to the type T if possible.
    ///
    /// # Errors
    /// If the type T cannot be constructed from the stored string value.
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

/// A HTTP Version struct. Can be HTTP1.1, HTTP2, HTTP3
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http2,
    Http3,
}
impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http10 => write!(f, "HTTP/1.0"),
            HttpVersion::Http11 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}

/// A HTTP Request structure.
///
/// Holds the request:
/// `Method`
/// `Url`
/// `Version`
/// `body`
/// `chunks` (in the case of Chunk requests)
#[derive(PartialEq)]
pub struct HttpRequest {
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
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
    /// Creates a new HTTP Request.
    /// Defaulting to HTTP version 1.1
    /// and empty headers and body with a default URL '\' for home index.
    pub fn new() -> Self {
        Self {
            version: HttpVersion::Http11,
            headers: Vec::new(),
            body: Vec::new(),
            method: HttpMethod::Get,
            url: "\\".to_string(),
            chunks: Vec::new(),
            chunked: false,
        }
    }

    /// Adds data to the a request.
    /// This method automatically calculates the content size and accounts for chunked requests.
    /// # Arguments
    /// `data` the slice of data being added to the body of this request.
    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
        if self.chunked {
            self.chunks.push((self.body.len(), data.len()));
            self.put_header(H_TRANSFER_ENCODING, "chunked");
        } else {
            self.put_header(H_CONTENT_LENGTH, self.body.len());
        }
    }

    /// Get a reference to the data vector contained in this request.
    pub fn data(&self) -> &Vec<u8> {
        &self.body
    }

    /// Get the version portion of this request.
    pub fn version(&self) -> HttpVersion {
        self.version
    }

    /// Get a vector of all the headers in this request.
    pub fn headers(&self) -> Vec<&HttpHeader> {
        self.headers.iter().collect()
    }

    /// Retrieve the value for a header with the give name.
    /// `name` the header being searched.
    ///
    /// # Returns an optional value if it exist.
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

    /// Ad a header to this request.
    /// Note that when  a header already exists, it's value is simply updated.
    ///
    /// As per the HTTP protocol, headers are not case sensitive.
    /// # Arguments
    /// `name` name for the header being added.
    /// `value` value for the header being added. It must implement Display so it can be turned into a string.
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

    /// Removes a header from this request if it exists.
    ///
    /// # Arguments
    /// `name` name for the header to search adn remove.
    /// The name is not case sensitive as the protocol specifies.
    pub fn remove_header(&mut self, name: &str) {
        self.headers
            .retain(|header| header.name.to_lowercase().ne(&name.to_lowercase()));
    }

    /// Retrieve the method for this request.
    pub fn method(&self) -> HttpMethod {
        self.method
    }

    /// Convert this request into a byte vector.
    /// Useful when transmitting a request across a communication medium.
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // first line, version + status code  + msg
        bytes.extend_from_slice(
            &format!("{} {} {}\r\n", self.method(), self.url, self.version()).into_bytes(),
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
    /// Create a HttpUrlBuilder to construct this URL
    pub fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::default()
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.url, self.version)?;
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

/// A HTTP Response structure.
///
/// Holds the request:
/// `Version`
/// `Status Code`
/// `Status Message`
/// `body`
/// `chunks` (in the case of Chunk requests)
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
    /// Create a new empty HTTP Response.
    ///
    /// Defaults to HTTP version 1.1
    /// Status code 200
    /// Empty body and body chunks
    pub fn new() -> HttpResponse {
        Self {
            version: HttpVersion::Http11,
            status_code: 200,
            status_msg: "Ok".to_string(),
            headers: Vec::new(),
            body: Vec::new(),
            chunks: Vec::new(),
            chunked: false,
        }
    }

    /// Insert data into the body of this response
    ///
    /// # Arguments
    /// `data` slice being copied to this response
    pub fn add_data(&mut self, data: &[u8]) {
        self.body.extend_from_slice(data);
        if self.chunked {
            self.chunks.push((self.body.len(), data.len()));
            self.put_header(H_TRANSFER_ENCODING, "chunked");
        } else {
            self.put_header(H_CONTENT_LENGTH, self.body.len());
        }
    }

    /// Retrieve all the data currently in this response.
    pub fn data(&self) -> &Vec<u8> {
        &self.body
    }

    /// Retrieve the version currently set in this response.
    pub fn version(&self) -> HttpVersion {
        self.version
    }

    /// Retrieve the status code in this response
    pub fn status_code(&self) -> usize {
        self.status_code
    }

    /// Retrieve status message string passed on the HTTP response.
    pub fn status_msg(&self) -> String {
        self.status_msg.clone()
    }

    /// Ad a header to this response.
    /// Note that when  a header already exists, it's value is simply updated.
    ///
    /// As per the HTTP protocol, headers are not case sensitive.
    /// # Arguments
    /// `name` name for the header being added.
    /// `value` value for the header being added. It must implement Display so it can be turned into a string.
    pub fn header(&self, name: &str) -> Option<&HttpHeader> {
        self.headers
            .iter()
            .find(|header| header.name.to_lowercase().eq(&name.to_lowercase()))
    }

    /// Removes a header from this response if it exists.
    ///
    /// # Arguments
    /// `name` name for the header to search adn remove.
    /// The name is not case sensitive as the protocol specifies.
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

    /// Retrieve the value for a header with the give name.
    /// `name` the header being searched.
    ///
    /// # Returns an optional value if it exist.
    pub fn headers(&self) -> Vec<&HttpHeader> {
        self.headers.iter().collect()
    }

    /// Convert this response into a byte vector.
    /// Useful when transmitting a request across a communication medium.
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

    /// Create a HttpUrlBuilder to construct this URL
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::default()
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

/// Builder for HTTP response.
///
/// Contains utility methods useful for creating a response.
///
/// # Example
/// ```no_run
///   use http_parse::HttpResponseBuilder;

///   let mut response = HttpResponseBuilder::new()
///         .header("Content-Type", "text/plain")
///         .header("Content-Length", 11)
///         .build();
/// ```
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
    /// Create a new HTTP Response builder.
    ///
    /// Defaults to HTTP 1.1 version.
    /// Status code 201 (ok)
    /// No data or chunks.
    pub fn new() -> Self {
        Self {
            version: Some(HttpVersion::Http11),
            status_code: Some(StatusCode::OK),
            headers: None,
            data: None,
            chunks: None,
        }
    }

    /// Set status code to this HTTP Response.
    pub fn status(mut self, code: StatusCode) -> Self {
        self.status_code = Some(code);
        self
    }

    /// Add a header to this HTTP Response
    ///
    /// # Arguments
    /// `name`  name of the header being added.
    /// `value` value being added for this header. Must have a type that implements `std::fmt::Display`
    ///
    pub fn header<T>(mut self, name: &str, value: T) -> Self
    where
        T: Display,
    {
        let headers = self.headers.get_or_insert(Vec::new());
        headers.push(HttpHeader::new(name, value));
        self
    }

    /// Add a body (data) to this HTTP Response
    ///
    /// # Arguments
    /// `new_data`  The data being added to this response
    pub fn body(mut self, new_data: &[u8]) -> Self {
        let data = self.data.get_or_insert(Vec::new());
        data.extend_from_slice(new_data);
        self
    }

    /// Construct a response from the given data.
    ///
    /// # Example
    ///```no_run
    /// # use http_parse::HttpResponseBuilder;
    ///   let mut response = HttpResponseBuilder::new()
    ///         .header("Content-Type", "text/plain")
    ///         .header("Content-Length", 11)
    ///         .build();
    /// ```
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

/// Builder for HTTP Request.
///
/// Contains utility methods useful for creating a Request.
///
/// # Example
/// ```no_run
/// use http_parse::H_RANGE;
/// use http_parse::HttpRequestBuilder;
/// use http_parse::H_HOST;
/// use http_parse::HttpMethod;
/// let mut request = HttpRequestBuilder::new()
///     .path("/")
///     .header(H_HOST, "192.168.1.8")
///     .header(H_RANGE, 0-5000/100000)
///     .build();
/// // code here
/// ```
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
    /// Create a new HTTP Request builder.
    ///
    /// Defaults to HTTP 1.1 version.
    /// GET  method
    /// "/" URL
    /// No data or chunks.
    pub fn new() -> Self {
        Self {
            method: Some(HttpMethod::Get),
            url: Some("/".to_string()),
            version: Some(HttpVersion::Http11),
            headers: None,
            data: None,
            chunks: None,
        }
    }

    /// Set a HTTP Method for this Request
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Add a header to this HTTP Request
    ///
    /// # Arguments
    /// `name`  name of the header being added.
    /// `value` value being added for this header. Must have a type that implements `std::fmt::Display`
    ///
    pub fn header<T>(mut self, name: &str, value: T) -> Self
    where
        T: std::fmt::Display,
    {
        let headers = self.headers.get_or_insert(Vec::new());
        headers.push(HttpHeader::new(name, value));
        self
    }

    /// Add a body (data) to this HTTP Request
    ///
    /// # Arguments
    /// `new_data`  The data being added to this Request
    pub fn body(mut self, new_data: &[u8]) -> Self {
        let data = self.data.get_or_insert(Vec::new());
        data.extend_from_slice(new_data);
        self
    }

    // Add a path to this HTTP Request.
    /// See also [`crate::HttpRequestBuilder::url`] method but can be given a string rather than [HttpUrl].
    ///
    /// # Arguments
    /// `url` URL being added
    pub fn path(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    /// Add a URL to this HTTP Request
    ///
    /// # Arguments
    /// `url` URL being added
    pub fn url(mut self, url: &HttpUrl) -> Self {
        self.url = Some(url.target());
        self
    }

    /// Construct a HTTP Request from the given data.
    ///
    /// # Example
    ///```no_run
    /// use http_parse::H_TRANSFER_ENCODING;
    /// use http_parse::H_RANGE;
    /// use http_parse::HttpRequestBuilder;
    /// use http_parse::H_HOST;
    /// use http_parse::HttpMethod;
    /// use std::net::TcpStream;
    /// use std::io::Write;
    /// fn main() -> Result<(), std::io::Error>{
    ///     let host = "localhost:8080";
    ///     let req = "/";
    ///     let mut client = TcpStream::connect(host)?;
    ///     let request = HttpRequestBuilder::new()
    ///        .method(HttpMethod::Head)
    ///        .path(req)
    ///        .header(H_HOST, "192.168.1.8")
    ///        .build();
    ///
    ///     client.write_all(&request.into_bytes())?;
    ///     Ok(())
    /// }
    /// ```
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
            url: url.to_string(),
        }
    }
}

impl TryFrom<&str> for HttpUrl {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// HTTP URL Structure
/// Provides methods to parse a URL as well as to create one.
///
/// # Example:
/// ```rust
/// # use http_parse::{HttpUrl};
/// let google_url = HttpUrl::builder()
///         .scheme("https")
///         .host("www.google.com")
///         .build();
/// ```
#[derive(Debug, Clone)]
pub struct HttpUrl {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
    query: HashMap<String, String>,
    fragment: Option<String>,
}
impl HttpUrl {
    /// Get the schema for this URL
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// Get the host for this URL
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Get the port for this URL if one exists
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// Get the path for this URL
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Retrieve the connection address
    pub fn address(&self) -> String {
        if let Some(port) = self.port() {
            format!("{}:{}", self.host(), port)
        } else {
            let port = if self.scheme.eq("https") {
                DEFAULT_HTTPS_PORT
            } else {
                DEFAULT_HTTP_PORT
            };
            format!("{}:{}", self.host, port)
        }
    }

    /// Get the query argument with the current key if available in this URL
    /// # Arguments
    /// `key` key to be searched
    pub fn query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    /// Get the fragment portion of this URL if available
    pub fn fragment(&self) -> Option<&String> {
        self.fragment.as_ref()
    }

    /// Get a file path from this URL if one is contained.
    pub fn file(&self) -> Option<&str> {
        if self.path.ends_with("/") || !self.path.contains('.') {
            None
        } else {
            if self.path.contains("?") || self.path.contains("#") {
                let file_path = self.path.split("/").last().unwrap();
                file_path.split(['?', '#']).nth(0)
            } else {
                self.path.split("/").last()
            }
        }
    }
    /// Get the URL's target. This contains the
    /// path + query arguments + fragment arguments if present.
    pub fn target(&self) -> String {
        let mut url = self.path.clone();

        if !self.query.is_empty() {
            let query_string: Vec<String> = self
                .query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&format!("?{}", query_string.join("&")));
        }

        if let Some(fragment) = &self.fragment {
            url.push_str(&format!("#{}", fragment));
        }

        url
    }

    /// Create a HttpUrlBuilder to construct this URL
    pub fn builder() -> HttpUrlBuilder {
        HttpUrlBuilder::default()
    }

    /// Parse a HTTP URL from a given string.
    pub fn parse(url: &str) -> Result<HttpUrl, &'static str> {
        let (scheme, remainder) = if let Some(pos) = url.find("://") {
            let (scheme, remainder) = url.split_at(pos);
            if scheme.eq("http") || scheme.eq("https") {
                (scheme.to_string(), &remainder[3..])
            } else {
                return Err("Invalid scheme provided, supported only `HTTP` and `HTTPS`");
            }
        } else {
            ("http".to_string(), url)
        };

        let mut host_parts = remainder.splitn(2, '/');
        let host_port = host_parts.next().unwrap();
        let path = format!("/{}", host_parts.next().unwrap_or(""));

        let (host, port) = if let Some(colon_pos) = host_port.find(':') {
            let host = &host_port[..colon_pos];
            let port = &host_port[colon_pos + 1..];
            (
                host.to_string(),
                Some(port.parse::<u16>().map_err(|_| "Invalid port")?),
            )
        } else {
            (host_port.to_string(), None)
        };

        let mut query = HashMap::new();
        let mut fragment = None;
        let mut path_without_query = &path[..];

        if let Some(fragment_pos) = path.find('#') {
            fragment = Some(path[fragment_pos + 1..].to_string());
            path_without_query = &path[..fragment_pos];
        }

        if let Some(query_pos) = path_without_query.find('?') {
            let query_string = &path_without_query[query_pos + 1..];
            path_without_query = &path_without_query[..query_pos];
            for kv in query_string.split('&') {
                let mut kv_parts = kv.split('=');
                let key = kv_parts.next().unwrap().to_string();
                let value = kv_parts.next().unwrap_or("").to_string();
                query.insert(key, value);
            }
        }

        Ok(HttpUrl {
            scheme,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

impl Display for HttpUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut url = format!("{}://{}", self.scheme, self.host);

        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }
        if !self.path.is_empty() {
            url.push('/');
            url.push_str(&self.path);
        }

        if !self.query.is_empty() {
            let query_string: Vec<String> = self
                .query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&format!("?{}", query_string.join("&")));
        }

        if let Some(fragment) = &self.fragment {
            url.push_str(&format!("#{}", fragment));
        }

        write!(f, "{}", url)
    }
}

#[derive(Debug)]
pub struct HttpUrlBuilder {
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
    query: HashMap<String, String>,
    fragment: Option<String>,
}

impl HttpUrlBuilder {
    /// Create a new Builder
    pub fn new() -> Self {
        Self {
            scheme: "http".to_string(), // Default scheme
            host: "".to_string(),
            port: None,
            path: "".to_string(),
            query: HashMap::new(),
            fragment: None,
        }
    }

    /// Assign a scheme to the URL.
    pub fn scheme(mut self, scheme: &str) -> Self {
        self.scheme = scheme.to_string();
        self
    }

    /// Assign a host to the URL.
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Assign a port to the URL.
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Assign a path to the URL
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    /// Assign a fragment to the URL.
    pub fn fragment(mut self, fragment: &str) -> Self {
        self.fragment = Some(fragment.to_string());
        self
    }

    /// Add a query key,value pair to the URL.
    pub fn param<T>(mut self, key: &str, value: &T) -> Self
    where
        T: Display,
    {
        self.query.insert(key.to_string(), value.to_string());
        self
    }

    /// Construct the URL from the given arguments.
    pub fn build(self) -> HttpUrl {
        HttpUrl {
            scheme: self.scheme,
            host: self.host,
            port: self.port,
            path: self.path,
            query: self.query,
            fragment: self.fragment,
        }
    }
}

impl Default for HttpUrlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum HttpParseError {
    Method(String),
    Version(String),
    Url(String),
    StatusCode(String),
    Header(String),
    Other(String),
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpParseError::Method(value) => write!(f, "Unsupported HTTP Method `{}`", value),
            HttpParseError::Version(value) => write!(f, "Unsupported HTTP Version `{}`", value),
            HttpParseError::Url(value) => write!(f, "Invalid HTTP URL `{}`", value),
            HttpParseError::StatusCode(value) => write!(f, "Invalid HTTP Status Code `{}`", value),
            HttpParseError::Header(value) => write!(f, "Error reading header `{}`", value),
            HttpParseError::Other(value) => write!(f, "Read error: `{}`", value),
        }
    }
}

impl core::error::Error for HttpParseError {}

impl From<std::io::Error> for HttpParseError {
    fn from(value: std::io::Error) -> Self {
        Self::Other(value.to_string())
    }
}
impl From<HttpParseError> for std::io::Error {
    fn from(value: HttpParseError) -> Self {
        match value {
            HttpParseError::Method(value) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, value)
            }
            HttpParseError::Version(value) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, value)
            }
            HttpParseError::Url(value) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, value)
            }
            HttpParseError::StatusCode(value) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, value)
            }
            HttpParseError::Header(value) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, value)
            }
            HttpParseError::Other(value) => std::io::Error::new(std::io::ErrorKind::Other, value),
        }
    }
}
