use std::{
    fs,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use http_parse::{
    HttpParser, HttpRequest, HttpResponse, H_ACCEPT_RANGES, H_CONNECTION, H_HOST, H_USER_AGENT,
};

pub struct Client;

impl Client {
    pub fn download(host: &str, req: &str) -> std::io::Result<()> {
        let mut client = TcpStream::connect(host)?;
        let mut request = HttpRequest::new().with_url(req);
        request.put_header(H_ACCEPT_RANGES, "bytes");
        request.put_header(H_CONNECTION, "close");
        request.put_header(H_HOST, "192.168.1.8");
        request.put_header(H_USER_AGENT, "Mozzila/5.0 (WD test)");
        client.write_all(&request.into_bytes())?;
        println!("== SENDING == \n{}", request);

        let mut parser = HttpParser::from_reader(&mut client);
        let response = parser.read_response()?;

        let mut file = fs::File::create(&format!(".{}", req))?;
        file.write_all(response.data())?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    Client::download("192.168.1.8:8080", "/Video.mp4")?;

    Ok(())
}
