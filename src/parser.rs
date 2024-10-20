use std::io::{BufRead, BufReader, ErrorKind, Read};

use crate::{
    HttpHeader, HttpMethod, HttpRequest, HttpResponse, HttpVersion, Url, H_CONTENT_LENGTH,
    H_TRANSFER_ENCODING,
};

/// A Parser for HTTP content.
/// Currently this implementation only follows HTTP 1.1.
/// This parser is a naive implementation of a parser of the HTTP protocol.
///
/// The parser supports partsing Responses from any structure that implements the `std::io::Read`` trait.
///
/// # Example:
/// ```no_run
///   use http_parse::ByteBuffer;
///   let request_text =
///         "GET / HTTP/1.1\r\nHost: developer.mozilla.org\r\nAccept-Language: fr\r\n\r\n";
///         
///  let mut reader = ByteBuffer::new(request_text.as_bytes());
///  let mut parser = http_parse::HttpParser::from_reader(&mut reader);
///  let request = parser.request().unwrap();
///  assert_eq!(&request.into_bytes(), request_text.as_bytes());
/// ```
///
pub struct HttpParser<'a, R> {
    reader: BufReader<&'a mut R>,
}

impl<'a, R: Read> HttpParser<'a, R> {
    /// Create a HTTP Parser from a reader that implements `std::io::Read`.
    pub fn from_reader(reader: &'a mut R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    /// Parse a `HttpResponse` by reading bytes in this reader/stream.
    ///
    /// The Response parsed through this methos includes:
    /// `HttpHeader`
    /// `HttpVersion`
    /// `StatusCode`
    /// `body data` and more.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn response(&mut self) -> std::io::Result<HttpResponse> {
        self.parse_response(true)
    }

    /// Parse a `HttpResponse` by reading bytes in this reader/stream.
    ///
    /// The Response parsed through this methos includes:
    /// `HttpHeader`
    /// `HttpVersion`
    /// `StatusCode`
    /// `body data` is skipped completely.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn response_head_only(&mut self) -> std::io::Result<HttpResponse> {
        self.parse_response(false)
    }

    fn parse_response(&mut self, include_data: bool) -> std::io::Result<HttpResponse> {
        let mut buffer = Vec::with_capacity(100);
        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let version = Self::parse_version(&buffer)?;
        buffer.clear();

        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let status_code = Self::parse_status_code(&buffer)?;
        buffer.clear();

        let _ = self.reader.read_until(b'\n', &mut buffer)?;
        let message = String::from_utf8_lossy(&buffer).trim().to_owned();
        buffer.clear();

        // let headers = self.parse_headers();
        let mut headers = Vec::new();
        self.parse_headers_two(&mut headers)?;
        let body = Vec::new();
        let chunks = Vec::new();
        let mut response = HttpResponse {
            version,
            status_code,
            status_msg: message,
            headers,
            body,
            chunks,
            chunked: false,
        };
        if include_data {
            let encoding_header = response.header(H_TRANSFER_ENCODING).cloned();
            let content_header = response.header(H_CONTENT_LENGTH).cloned();

            self.extract_body_data(
                encoding_header,
                content_header,
                &mut response.chunks,
                &mut response.body,
            )?;

            response.chunked = !response.chunks.is_empty();
        }
        Ok(response)
    }

    /// Parse a `HttpRequest` by reading bytes in this reader/stream.
    ///
    /// The Request parsed through this methos includes:
    /// `HttpHeader`
    /// `HttpMethod`
    /// `Requested URL`
    /// `body data` and more.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn request(&mut self) -> std::io::Result<HttpRequest> {
        self.parse_request(true)
    }

    /// Parse a `HttpRequest` by reading bytes in this reader/stream.
    ///
    /// The Request parsed through this methos includes:
    /// `HttpHeader`
    /// `HttpMethod`
    /// `Requested URL`
    /// `body data` is skipped completely.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn request_head_only(&mut self) -> std::io::Result<HttpRequest> {
        self.parse_request(false)
    }
    pub fn parse_request(&mut self, include_data: bool) -> std::io::Result<HttpRequest> {
        let mut buffer = Vec::with_capacity(100);
        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let method = Self::parse_method(&buffer)?;
        buffer.clear();

        let _ = self.reader.read_until(b' ', &mut buffer)?;

        let url = String::from_utf8_lossy(&buffer).trim().to_owned();
        buffer.clear();

        let _ = self.reader.read_until(b'\n', &mut buffer)?;

        let version = Self::parse_version(&buffer)?;
        // let headers = self.parse_headers();
        let mut headers = Vec::new();
        self.parse_headers_two(&mut headers)?;

        let body = Vec::new();
        let chunks = Vec::new();

        let mut request = HttpRequest {
            method,
            url: Url { inner: url },
            version,
            headers,
            body,
            chunked: false,
            chunks,
        };
        if include_data {
            let encoding_header = request.header(H_TRANSFER_ENCODING).cloned();
            let content_header = request.header(H_CONTENT_LENGTH).cloned();

            self.extract_body_data(
                encoding_header,
                content_header,
                &mut request.chunks,
                &mut request.body,
            )?;

            request.chunked = !request.chunks.is_empty();
        }
        Ok(request)
    }

    fn extract_body_data(
        &mut self,
        encoding_header: Option<HttpHeader>,
        content_header: Option<HttpHeader>,
        chunks: &mut Vec<(usize, usize)>,
        body: &mut Vec<u8>,
    ) -> Result<(), std::io::Error> {
        let mut chunked = false;
        encoding_header.inspect(|h| {
            if !h.value.contains("identity") {
                chunked = true;
            }
        });
        if chunked {
            self.read_chunked_body(body, chunks)?;
        } else if let Some(header) = content_header {
            match header.value::<usize>() {
                Ok(length) => {
                    body.resize_with(length, || 0);
                    self.reader.read_exact(body)?;
                }
                Err(_e) => {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Ivalid header `{}`", header).as_str(),
                    ))?;
                }
            };
        }

        Ok(())
    }

    fn read_chunked_body(
        &mut self,
        body: &mut Vec<u8>,
        chunks: &mut Vec<(usize, usize)>,
    ) -> Result<(), std::io::Error> {
        let mut buff = Vec::with_capacity(16);
        while let Ok(n) = self.reader.read_until(b'\n', &mut buff) {
            // done reading
            if n == 0 {
                buff.clear();
                // done reading
                break;
            }

            // parse hex byte numbers contained in chunk
            let digits_str = String::from_utf8_lossy(buff.trim_ascii()).to_string();
            match usize::from_str_radix(&digits_str, 16) {
                Ok(chunk_size) => {
                    if chunk_size == 0 {
                        let _ = self.reader.read_until(b'\n', &mut buff);
                        break;
                    } else {
                        let mut chunk_buff = vec![0; chunk_size];
                        self.reader.read_exact(&mut chunk_buff)?;

                        chunks.push((body.len(), body.len() + chunk_buff.len()));
                        body.extend_from_slice(&chunk_buff);
                    }
                }
                Err(_) => break, // invalid reading of body for now just exit loop
            }
            // ignore new line after chunk
            let _ = self.reader.read_until(b'\n', &mut buff);
            buff.clear();
        }
        // last chunk 0 data
        if !chunks.is_empty() {
            chunks.push((0, 0));
        }
        Ok(())
    }

    fn parse_method(method: &[u8]) -> std::io::Result<HttpMethod> {
        match method.trim_ascii() {
            b"GET" => Ok(HttpMethod::Get),
            b"POST" => Ok(HttpMethod::Post),
            b"PUT" => Ok(HttpMethod::Put),
            b"HEAD" => Ok(HttpMethod::Head),
            b"OPTIONS" => Ok(HttpMethod::Options),
            b"DELETE" => Ok(HttpMethod::Delete),
            b"TRACE" => Ok(HttpMethod::Trace),

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Invalid HTTP method in buffer `{}`.",
                    String::from_utf8_lossy(method)
                ),
            )),
        }
    }

    fn parse_version(version: &[u8]) -> std::io::Result<HttpVersion> {
        match version.trim_ascii() {
            b"HTTP/1.1" => Ok(HttpVersion::Http11),
            // b"HTTP/2" => Ok(HttpVersion::Http2),
            // b"HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Unsuported HTTP version in buffer `{}`.",
                    String::from_utf8_lossy(version.trim_ascii())
                ),
            )),
        }
    }

    fn parse_status_code(status_code: &[u8]) -> std::io::Result<usize> {
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
        let mut headers = Vec::with_capacity(100);
        let mut line = String::new();
        while self.reader.read_line(&mut line).is_ok() {
            // empty line between request and body, we are done
            if line.trim().is_empty() {
                break;
            } else {
                if let Some(index) = line.find(':') {
                    if index < line.len() {
                        let name = line[0..index].to_string().trim().to_string();
                        let value = line[index + 1..line.len()]
                            .trim_ascii_start()
                            .replace(['\r', '\n'], "")
                            .to_string();
                        headers.push(HttpHeader { name, value });
                    }
                }
                line.clear();
            }
        }
        headers
    }

    fn parse_headers_two(&mut self, headers: &mut Vec<HttpHeader>) -> std::io::Result<()> {
        while !self.is_line_end()? {
            let mut name = Vec::new();
            let mut value = Vec::new();
            let name_len = self.reader.read_until(b':', &mut name)?;
            self.skip_matching(|byte| (byte as char).is_whitespace())?;
            let value_len = self.reader.read_until(b'\n', &mut value)?;
            headers.push(HttpHeader::new(
                String::from_utf8_lossy(&name[0..name_len - 1]),
                String::from_utf8_lossy(&value[0..value_len - 2]),
            ));
        }
        self.skip_next_line();
        Ok(())
    }

    fn skip_matching<F>(&mut self, f: F) -> std::io::Result<usize>
    where
        F: Fn(u8) -> bool,
    {
        let mut read = 0;
        loop {
            let (done, used) = {
                let available = match self.reader.fill_buf() {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
                match available.iter().position(|byte| !f(*byte)) {
                    Some(index) => (true, index),
                    None => (false, 0),
                }
            };
            self.reader.consume(used);
            read += used;
            if done || used == 0 {
                return Ok(read);
            }
        }
    }

    fn is_line_end(&mut self) -> std::io::Result<bool> {
        if self.reader.buffer().len() >= 2 {
            Ok(self.reader.buffer().starts_with(b"\r\n"))
        } else {
            loop {
                match self.reader.fill_buf() {
                    Ok(available) => {
                        return Ok(available.is_empty() || available.starts_with(b"\r\n"))
                    }
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                    Err(e) => return Err(e),
                };
            }
        }
    }

    fn skip_next_line(&mut self) -> std::io::Result<()> {
        if self.is_line_end()? {
            self.reader.consume(2);
        }
        Ok(())
    }
}

/// A ByteBuffer data container which implements Read.
/// Useful for when HTTP data is exist in memory andthe HttpParser is used for parsing it.
///
/// # Example:
/// ```no_run
/// # use http_parse::ByteBuffer;
/// # use http_parse::H_TRANSFER_ENCODING;
/// # use http_parse::H_HOST;
/// # use http_parse::HttpRequestBuilder;
/// # use http_parse::HttpHeader;
/// let range_value = "0-5000/10000";
/// let response_text="HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nMozilla\r\n11\r\nDeveloper Network\r\n0\r\n\r\n";
///
/// let mut reader = ByteBuffer::new(response_text.as_bytes());
/// let mut parser = http_parse::HttpParser::from_reader(&mut reader);
/// let response = parser.response().unwrap();
/// let transfer_header = HttpHeader::new("Transfer-Encoding", "chunked");
///
/// assert_eq!(Some(&transfer_header), response.header(H_TRANSFER_ENCODING));
/// assert_eq!(response.data(), b"MozillaDeveloper Network");
/// assert_eq!(response.into_bytes(), response_text.as_bytes());
/// ```
pub struct ByteBuffer {
    inner: Vec<u8>,
    index: usize,
}
#[allow(dead_code)]
impl ByteBuffer {
    pub fn new(buffer: &[u8]) -> Self {
        Self {
            inner: buffer.to_vec(),
            index: 0,
        }
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
