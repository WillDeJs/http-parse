use http_parse::{
    ByteBuffer, HttpHeader, HttpMethod, HttpParser, HttpVersion, StatusCode, H_TRANSFER_ENCODING,
};

#[test]
fn test_response() {
    let response = "HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r
Date: Fri, 21 Jun 2024 14:18:33 GMT\r
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT\r
Content-Length: 45\r
\r
<!doctype html>
<!-- HTML content follows -->";
    let mut reader = ByteBuffer::new(response.as_bytes());
    let mut parser = HttpParser::from_reader(&mut reader);
    let response = parser.response().unwrap();
    assert_eq!(response.version(), HttpVersion::Http11);
    assert_eq!(response.status_code(), 200);
    assert_eq!(response.status_msg(), "OK".to_string());
    let data = r"<!doctype html>
<!-- HTML content follows -->";
    let date = "Fri, 21 Jun 2024 14:18:33 GMT";
    let my_header = HttpHeader::new("date", date);
    assert_eq!(response.data(), data.as_bytes());
    assert_eq!(Some(&my_header), response.header("date"));
}
#[test]
fn test_response_bytes() {
    let response_text = "HTTP/1.1 200 OK\r
Content-Type: text/html; charset=UTF-8\r
Date: Fri, 21 Jun 2024 14:18:33 GMT\r
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT\r
Content-Length: 44\r
\r
<!doctype html><!-- HTML content follows -->";

    let mut reader = ByteBuffer::new(response_text.as_bytes());
    let mut parser = HttpParser::from_reader(&mut reader);
    let response = parser.response().unwrap();
    assert_eq!(response_text.as_bytes(), &response.into_bytes());
}

#[test]
fn test_request() {
    let request = "GET / HTTP/1.1\r
Host: developer.mozilla.org\r
Accept-Language: fr\r\n";

    let mut reader = ByteBuffer::new(request.as_bytes());
    let mut parser = HttpParser::from_reader(&mut reader);
    let request = parser.request().unwrap();

    assert_eq!(request.version(), HttpVersion::Http11);
    assert_eq!(request.method(), HttpMethod::Get);
    let my_header = HttpHeader::new("host", "developer.mozilla.org");
    assert_eq!(request.header("host"), Some(&my_header));
}
#[test]
fn test_request_bytes() {
    let request_text =
        "GET / HTTP/1.1\r\nHost: developer.mozilla.org\r\nAccept-Language: fr\r\n\r\n";

    let mut reader = ByteBuffer::new(request_text.as_bytes());
    let mut parser = http_parse::HttpParser::from_reader(&mut reader);
    let request = parser.request().unwrap();
    assert_eq!(&request.into_bytes(), request_text.as_bytes());
}

#[test]
fn test_response_body_chuncked() {
    let response_text="HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\n\r\n7\r\nMozilla\r\n11\r\nDeveloper Network\r\n0\r\n\r\n";

    let mut reader = ByteBuffer::new(response_text.as_bytes());
    let mut parser = http_parse::HttpParser::from_reader(&mut reader);
    let response = parser.response().unwrap();
    let transfer_header = HttpHeader::new("Transfer-Encoding", "chunked");

    assert_eq!(Some(&transfer_header), response.header(H_TRANSFER_ENCODING));
    assert_eq!(response.data(), b"MozillaDeveloper Network");
    assert_eq!(response.into_bytes(), response_text.as_bytes());
}

#[test]
fn test_status_code_conversion() {
    assert_eq!(StatusCode::OK, 200);
    assert_eq!(200, StatusCode::OK);
}
