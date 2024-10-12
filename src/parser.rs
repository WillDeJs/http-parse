use std::io::{BufRead, BufReader, Read};

use crate::{HttpHeader, HttpMethod, HttpRequest, HttpResponse, HttpVersion, Url};

pub struct HttpParser<R> {
    reader: BufReader<R>,
}

impl<R: Read> HttpParser<R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
        }
    }

    pub fn read_request(&mut self) -> std::io::Result<HttpRequest> {
        let mut buffer = Vec::new();
        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let method = Self::parse_method(&buffer)?;
        buffer.clear();

        let _ = self.reader.read_until(b' ', &mut buffer)?;

        let url = String::from_utf8_lossy(&buffer).trim().to_owned();
        buffer.clear();

        let _ = self.reader.read_until(b'\n', &mut buffer)?;

        let version = Self::parse_version(&buffer)?;
        let headers = self.parse_headers();

        if let Some(content_length) = headers
            .iter()
            .find(|head| head.name.to_lowercase().eq("content-length"))
        {
            // Enforce reading the contents based on a Content-Type
            let size = content_length.value.parse::<usize>().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid Content-Value:{} set.", content_length.value).as_str(),
                )
            })?;
            let mut body = vec![0;size];
            let size = self.reader.read(&mut body)?;
            body.truncate(size);
            Ok(HttpRequest {
                method,
                url: Url { inner: url },
                version,
                headers,
                body,
            })
        } else {
            // We don't have content length, read until server closes connection
            let mut body = Vec::new();
            self.reader.read_to_end(&mut body)?;
            Ok(HttpRequest {
                method,
                url: Url { inner: url },
                version,
                headers,
                body,
            })
        }
    }

    pub fn read_response(&mut self) -> std::io::Result<HttpResponse> {
        let mut buffer = Vec::new();

        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let version = Self::parse_version(&buffer)?;
        buffer.clear();

        let _ = self.reader.read_until(b' ', &mut buffer)?;
        let status_code = Self::parse_status_code(&buffer)?;
        buffer.clear();
        
        let _ = self.reader.read_until(b'\n', &mut buffer)?;
        let message = String::from_utf8_lossy(&buffer).trim().to_owned();
        buffer.clear();
        
        let headers = self.parse_headers();
        if let Some(content_length) = headers
            .iter()
            .find(|head| head.name.to_lowercase().eq("content-length"))
        {
            // Enforce reading the contents based on a Content-Type
            let size = content_length.value.parse::<usize>().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid Content-Value:{} set.", content_length.value).as_str(),
                )
            })?;
            let mut body = vec![0;size];
            let size = self.reader.read(&mut body)?;
            body.truncate(size);
            Ok(HttpResponse {
                version,
                status_code,
                status_msg: message,
                headers,
                body,
            })
        } else {
            // We don't have content length, read until server closes connection
            let mut body = Vec::new();
            let _ = self.reader.read_to_end(&mut body)?;
            Ok(HttpResponse {
                version,
                status_code,
                status_msg: message,
                headers,
                body,
            })
        }
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
        let mut headers = Vec::new();
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
}

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
