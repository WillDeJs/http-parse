use std::io::{BufRead, BufReader, Read};

use crate::{
    types::HttpParseError, HttpHeader, HttpMethod, HttpRequest, HttpResponse, HttpVersion,
    H_CONTENT_LENGTH, H_TRANSFER_ENCODING,
};

/// A Parser for HTTP content.
/// Currently this implementation only follows HTTP 1.1.
/// This parser is a naive implementation of a parser of the HTTP protocol.
///
/// The parser supports parsing Responses from any structure that implements the `std::io::Read`` trait.
///
/// # Example:
/// ```no_run
///   use std::io::Cursor;
///   let request_text =
///         "GET / HTTP/1.1\r\nHost: developer.mozilla.org\r\nAccept-Language: fr\r\n\r\n";
///         
///  let mut reader = Cursor::new(request_text.as_bytes());
///  let mut parser = http_parse::HttpParser::from_reader(&mut reader);
///  let request = parser.request().unwrap();
///  assert_eq!(&request.into_bytes(), request_text.as_bytes());
/// ```
///
pub struct HttpParser<'a, R> {
    reader: BufReader<&'a mut R>,
    max_body_size: usize,
}

const DEFAULT_MAX_BODY_SIZE: usize = 64 * 1024 * 1024;

impl<'a, R: Read> HttpParser<'a, R> {
    /// Create a HTTP Parser from a reader that implements `std::io::Read`.
    pub fn from_reader(reader: &'a mut R) -> Self {
        Self {
            reader: BufReader::new(reader),
            max_body_size: DEFAULT_MAX_BODY_SIZE,
        }
    }

    /// Set the maximum number of body bytes this parser will read or allocate.
    pub fn set_max_body_size(&mut self, max_body_size: usize) {
        self.max_body_size = max_body_size;
    }

    /// Parse a `HttpResponse` by reading bytes in this reader/stream.
    ///
    /// The Response parsed through this methods includes:
    /// `HttpHeader`
    /// `HttpVersion`
    /// `StatusCode`
    /// `body data` and more.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn response(&mut self) -> Result<HttpResponse, HttpParseError> {
        self.parse_response(true)
    }

    /// Parse a `HttpResponse` by reading bytes in this reader/stream.
    ///
    /// The Response parsed through this methods includes:
    /// `HttpHeader`
    /// `HttpVersion`
    /// `StatusCode`
    /// `body data` is skipped completely.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn response_head_only(&mut self) -> Result<HttpResponse, HttpParseError> {
        self.parse_response(false)
    }

    fn parse_response(&mut self, include_data: bool) -> Result<HttpResponse, HttpParseError> {
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

            if !(100..200).contains(&response.status_code)
                && !matches!(response.status_code, 204 | 205 | 304)
            {
                self.extract_body_data(
                    encoding_header,
                    content_header,
                    true,
                    &mut response.chunks,
                    &mut response.body,
                )?;
            }

            response.chunked = !response.chunks.is_empty();
        }
        Ok(response)
    }

    /// Parse a `HttpRequest` by reading bytes in this reader/stream.
    ///
    /// The Request parsed through this methods includes:
    /// `HttpHeader`
    /// `HttpMethod`
    /// `Requested URL`
    /// `body data` and more.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn request(&mut self) -> Result<HttpRequest, HttpParseError> {
        self.parse_request(true)
    }

    /// Parse a `HttpRequest` by reading bytes in this reader/stream.
    ///
    /// The Request parsed through this methods includes:
    /// `HttpHeader`
    /// `HttpMethod`
    /// `Requested URL`
    /// `body data` is skipped completely.
    ///
    /// # Errors:
    /// When reading from the Reader produces any error or the data provided is not formatted properly.
    pub fn request_head_only(&mut self) -> Result<HttpRequest, HttpParseError> {
        self.parse_request(false)
    }
    pub fn parse_request(&mut self, include_data: bool) -> Result<HttpRequest, HttpParseError> {
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
            url,
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
                false,
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
        read_to_eof: bool,
        chunks: &mut Vec<(usize, usize)>,
        body: &mut Vec<u8>,
    ) -> Result<(), HttpParseError> {
        if let Some(header) = encoding_header {
            let final_encoding = header.value.split(',').next_back().unwrap_or("").trim();
            if !final_encoding.eq_ignore_ascii_case("chunked") {
                return Err(HttpParseError::Header(format!(
                    "unsupported Transfer-Encoding: {}",
                    header.value
                )));
            }
            self.read_chunked_body(body, chunks)?;
        } else if let Some(header) = content_header {
            match header.value::<usize>() {
                Ok(length) => {
                    if length > self.max_body_size {
                        return Err(HttpParseError::Other(format!(
                            "body exceeds configured maximum of {} bytes",
                            self.max_body_size
                        )));
                    }
                    body.try_reserve_exact(length).map_err(|error| {
                        HttpParseError::Other(format!("unable to allocate body: {error}"))
                    })?;
                    body.resize_with(length, || 0);
                    self.reader.read_exact(body)?;
                }
                Err(_e) => Err(HttpParseError::Header(header.to_string()))?,
            };
        } else if read_to_eof {
            let mut buffer = [0; 8192];
            loop {
                let read = self.reader.read(&mut buffer)?;
                if read == 0 {
                    break;
                }
                if read > self.max_body_size.saturating_sub(body.len()) {
                    return Err(HttpParseError::Other(format!(
                        "body exceeds configured maximum of {} bytes",
                        self.max_body_size
                    )));
                }
                body.try_reserve(read).map_err(|error| {
                    HttpParseError::Other(format!("unable to allocate body: {error}"))
                })?;
                body.extend_from_slice(&buffer[..read]);
            }
        }

        Ok(())
    }

    fn read_chunked_body(
        &mut self,
        body: &mut Vec<u8>,
        chunks: &mut Vec<(usize, usize)>,
    ) -> Result<(), HttpParseError> {
        loop {
            let mut size_line = Vec::new();
            if self.reader.read_until(b'\n', &mut size_line)? == 0 || !size_line.ends_with(b"\r\n")
            {
                return Err(HttpParseError::Header(
                    "incomplete chunk size line".to_string(),
                ));
            }

            let size_text = String::from_utf8_lossy(&size_line[..size_line.len() - 2]);
            let size_text = size_text.split(';').next().unwrap_or("").trim();
            let chunk_size = usize::from_str_radix(size_text, 16)
                .map_err(|_| HttpParseError::Header("invalid chunk size".to_string()))?;

            if chunk_size == 0 {
                loop {
                    let mut trailer = Vec::new();
                    if self.reader.read_until(b'\n', &mut trailer)? == 0
                        || !trailer.ends_with(b"\r\n")
                    {
                        return Err(HttpParseError::Header(
                            "incomplete chunk trailer".to_string(),
                        ));
                    }
                    if trailer == b"\r\n" {
                        chunks.push((0, 0));
                        return Ok(());
                    }
                }
            }

            if chunk_size > self.max_body_size.saturating_sub(body.len()) {
                return Err(HttpParseError::Other(format!(
                    "body exceeds configured maximum of {} bytes",
                    self.max_body_size
                )));
            }

            let start = body.len();
            body.try_reserve_exact(chunk_size).map_err(|error| {
                HttpParseError::Other(format!("unable to allocate body: {error}"))
            })?;
            body.resize(start + chunk_size, 0);
            self.reader.read_exact(&mut body[start..])?;

            let mut terminator = [0; 2];
            self.reader.read_exact(&mut terminator)?;
            if terminator != *b"\r\n" {
                return Err(HttpParseError::Header(
                    "invalid chunk terminator".to_string(),
                ));
            }
            chunks.push((start, body.len()));
        }
    }

    fn parse_method(method: &[u8]) -> Result<HttpMethod, HttpParseError> {
        match method.trim_ascii() {
            b"GET" => Ok(HttpMethod::Get),
            b"POST" => Ok(HttpMethod::Post),
            b"PUT" => Ok(HttpMethod::Put),
            b"HEAD" => Ok(HttpMethod::Head),
            b"OPTIONS" => Ok(HttpMethod::Options),
            b"DELETE" => Ok(HttpMethod::Delete),
            b"TRACE" => Ok(HttpMethod::Trace),
            _ => Err(HttpParseError::Method(
                String::from_utf8_lossy(method).to_string(),
            )),
        }
    }

    fn parse_version(version: &[u8]) -> Result<HttpVersion, HttpParseError> {
        match version.trim_ascii() {
            b"HTTP/1.0" => Ok(HttpVersion::Http10),
            b"HTTP/1.1" => Ok(HttpVersion::Http11),
            b"HTTP/2" => Ok(HttpVersion::Http2),
            b"HTTP/3" => Ok(HttpVersion::Http3),
            _ => Err(HttpParseError::Version(
                String::from_utf8_lossy(version.trim_ascii()).to_string(),
            )),
        }
    }

    fn parse_status_code(status_code: &[u8]) -> Result<usize, HttpParseError> {
        let code_string = String::from_utf8_lossy(status_code);
        match code_string.trim().parse::<usize>() {
            Ok(value) => Ok(value),
            _ => Err(HttpParseError::StatusCode(
                String::from_utf8_lossy(status_code).to_string(),
            )),
        }
    }

    fn parse_headers_two(&mut self, headers: &mut Vec<HttpHeader>) -> Result<(), HttpParseError> {
        loop {
            let mut line = Vec::new();
            let line_len = self.reader.read_until(b'\n', &mut line)?;
            if line_len == 0 {
                return Ok(());
            }
            if !line.ends_with(b"\n") {
                return Err(HttpParseError::Header("incomplete header line".to_string()));
            }
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                return Ok(());
            }

            let separator = line
                .iter()
                .position(|byte| *byte == b':')
                .filter(|index| *index > 0)
                .ok_or_else(|| HttpParseError::Header("invalid header line".to_string()))?;
            let name = String::from_utf8_lossy(&line[..separator]);
            let value = String::from_utf8_lossy(&line[separator + 1..]);
            headers.push(HttpHeader::new(name.trim(), value.trim()));
        }
    }
}
