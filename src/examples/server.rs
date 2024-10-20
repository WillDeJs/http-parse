use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use http_parse::{threadpool, HttpParser, HttpResponseBuilder};

pub struct Server {
    inner: TcpListener,
}
impl Server {
    pub fn listen(port: u16) -> std::io::Result<Self> {
        let server = TcpListener::bind(format!("0.0.0.0:{}", port))?;
        Ok(Self { inner: server })
    }

    pub fn start<F>(&mut self, f: F) -> std::io::Result<()>
    where
        F: Fn(TcpStream),
        F: Send + Sync + 'static,
    {
        let pool = threadpool::ThreadPool::new(4);
        let fun_arc = Arc::new(f);
        for client in self.inner.incoming().flatten() {
            let fun_arc = fun_arc.clone();
            pool.execute(move || {
                fun_arc(client);
            });
        }

        Ok(())
    }
}

fn handle_connection(mut client: std::net::TcpStream) -> Result<(), std::io::Error> {
    let client_addr = client.peer_addr()?;
    let mut parser = HttpParser::from_reader(&mut client);
    let request = parser.request()?;
    print!(
        "====> Request from: {}:{} <====\n{}",
        client_addr.ip(),
        client_addr.port(),
        request
    );
    let mut response = HttpResponseBuilder::new()
        .header("Content-Type", "text/plain")
        .header("Content-Length", 11)
        .build();

    response.put_header("Content-Type", "text/plain");
    response.put_header("Content-Length", 11);
    response.add_data("Hello world".as_bytes());
    client.write_all(&response.into_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut server = Server::listen(8080)?;
    println!("Listening on port {}", 8080);
    server
        .start(|client| {
            handle_connection(client).expect("handle connection");
        })
        .expect("Starting connections");

    Ok(())
}
