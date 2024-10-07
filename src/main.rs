use http_parse::*;
fn main() {
    println!("Hello, world!");

    test_request();
}
fn test_request() {
    let request = r#"GET / HTTP/1.1
Host: developer.mozilla.org
Accept-Language: fr"#;

    let request = crate::HttpRequest::from_bytes(request.as_bytes()).unwrap();
    print!("{}", request);
    assert_eq!(request.version(), HttpVersion::Http1);
    assert_eq!(request.method(), HttpMethod::Get);
    let my_header = HttpHeader {
        name: "host".to_string(),
        value: "developer.mozilla.org".to_string(),
    };
    assert_eq!(request.header("host"), Some(&my_header));
}
