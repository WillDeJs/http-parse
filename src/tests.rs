use crate::{parser::ByteBuffer, HttpHeader, HttpMethod, HttpVersion};

#[test]
fn test_response() {
    let response = "HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r
Date: Fri, 21 Jun 2024 14:18:33 GMT\r
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT\r
Content-Length: 1234\r
\r
<!doctype html>
<!-- HTML content follows -->";
    let reader = ByteBuffer::new(response.as_bytes());
    let mut parser = crate::HttpParser::from_reader(reader);
    let response = parser.read_response().unwrap();
    assert_eq!(response.version(), HttpVersion::Http1);
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.status_msg(), "OK".to_string());
    let data = r"<!doctype html>
<!-- HTML content follows -->";
    let date = "Fri, 21 Jun 2024 14:18:33 GMT";
    let my_header = HttpHeader {
        name: "date".to_string(),
        value: date.to_string(),
    };
    assert_eq!(String::from_utf8_lossy(response.data()), data.to_string());
    assert_eq!(Some(&my_header), response.header("date"));
}
#[test]
fn test_response_bytes() {
    let response_text = "HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r
Date: Fri, 21 Jun 2024 14:18:33 GMT\r
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT\r
Content-Length: 1234\r
\r
<!doctype html><!-- HTML content follows -->";

    let reader = ByteBuffer::new(response_text.as_bytes());
    let mut parser = crate::HttpParser::from_reader(reader);
    let response = parser.read_response().unwrap();
    assert_eq!(response_text.as_bytes(), &response.into_bytes());
}

#[test]
fn test_request() {
    let request = "GET / HTTP/1.1\r
Host: developer.mozilla.org\r
Accept-Language: fr\r\n";

    let reader = ByteBuffer::new(request.as_bytes());
    let mut parser = crate::HttpParser::from_reader(reader);
    let request = parser.read_request().unwrap();

    assert_eq!(request.version(), HttpVersion::Http1);
    assert_eq!(request.method(), HttpMethod::Get);
    let my_header = HttpHeader {
        name: "host".to_string(),
        value: "developer.mozilla.org".to_string(),
    };
    assert_eq!(request.header("host"), Some(&my_header));
}
#[test]
fn test_request_bytes() {
    let request_text = "GET / HTTP/1.1\r\nHost: developer.mozilla.org\r\nAccept-Language: fr\r\n";

    let reader = ByteBuffer::new(request_text.as_bytes());
    let mut parser = crate::HttpParser::from_reader(reader);
    let request = parser.read_request().unwrap();
    assert_eq!(&request.into_bytes(), request_text.as_bytes());
}
