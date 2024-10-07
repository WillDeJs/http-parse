use crate::{HttpHeader, HttpMethod, HttpVersion};

#[test]
fn test_response() {
    let response = r#"HTTP/1.1 200 OK
Content-Type: text/html; charset=UTF-8
Date: Fri, 21 Jun 2024 14:18:33 GMT
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT
Content-Length: 1234

<!doctype html>
<!-- HTML content follows -->"#;

    let mut parser = crate::HttpParser::new(response.as_bytes());
    let response = parser.parse_response().unwrap();
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
fn test_request() {
    let request = r#"GET / HTTP/1.1
Host: developer.mozilla.org
Accept-Language: fr"#;

    let request = crate::HttpRequest::from_bytes(request.as_bytes()).unwrap();
    assert_eq!(request.version(), HttpVersion::Http1);
    assert_eq!(request.method(), HttpMethod::Get);
    let my_header = HttpHeader {
        name: "host".to_string(),
        value: "developer.mozilla.org".to_string(),
    };
    assert_eq!(request.header("host"), Some(&my_header));
}
