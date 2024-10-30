use std::{io::Write, net::TcpStream};

use http_parse::{
    HttpMethod, HttpParser, HttpRequestBuilder, HttpUrl, StatusCode, H_CONTENT_LENGTH,
    H_CONTENT_RANGE, H_HOST, H_RANGE, H_USER_AGENT, S_PARTIAL_CONTENT,
};

const MAX_CHUNK_SIZE: usize = 1_000_000; // 1 MB
pub struct Client;

impl Client {
    fn get_file_size(url: &HttpUrl) -> std::io::Result<usize> {
        let mut client = TcpStream::connect(url.address())?;
        let request = HttpRequestBuilder::new()
            .method(HttpMethod::Head)
            .path(url.path())
            .header(H_USER_AGENT, "Mozilla/5.0 (WD TEST)")
            .header(H_HOST, url.host())
            .build();
        println!("{}", request);
        client.write_all(&request.into_bytes())?;
        let mut parser = HttpParser::from_reader(&mut client);
        let response = parser.response_head_only()?;
        print!("{}", response);
        if response.status_code() != StatusCode::OK {
            eprint!(
                "Unexpected status code `{}` from server",
                response.status_code()
            );
            return Ok(0);
        }
        if let Some(header) = response.header(H_CONTENT_LENGTH) {
            Ok(header.value::<usize>().unwrap_or(0))
        } else {
            Ok(0)
        }
    }
    fn one_shot_download(url: &HttpUrl) -> std::io::Result<()> {
        let mut client = TcpStream::connect(url.address())?;
        let file = url.file().unwrap_or(url.path());
        let request = HttpRequestBuilder::new()
            .method(HttpMethod::Get)
            .path(url.path())
            .header(H_USER_AGENT, "Mozilla/5.0 (WD TEST)")
            .header(H_HOST, url.host())
            .build();
        client.write_all(&request.into_bytes())?;
        let response = HttpParser::from_reader(&mut client).response()?;
        if response.status_code() != StatusCode::OK {
            eprint!(
                "Unexpected status code `{}` from server",
                response.status_code()
            );
            return Ok(());
        }
        let mut out_file = std::fs::File::create(file)?;
        out_file.write_all(response.data())?;
        Ok(())
    }

    fn ranged_download(url: &HttpUrl, size: usize) -> std::io::Result<()> {
        let mut start_byte_index = 0;
        let mut chunk_size = std::cmp::min(MAX_CHUNK_SIZE, size);
        let file = url.file().unwrap_or(url.path());

        let mut out_file = std::fs::File::create(file)?;
        let mut total_written = 0;

        let range_value = format!("bytes={}-{}", start_byte_index, chunk_size);
        let mut request = HttpRequestBuilder::new()
            .path(url.path())
            .header(H_HOST, url.host())
            .header(H_RANGE, range_value)
            .header(H_USER_AGENT, "Mozilla/5.0 (WD TEST)")
            .build();
        println!("{}", request);
        loop {
            let mut client = TcpStream::connect(url.address())?;
            client.write_all(&request.into_bytes())?;
            let response = HttpParser::from_reader(&mut client).response()?;
            let status_code = response.status_code();
            if status_code != S_PARTIAL_CONTENT {
                eprint!(
                    "Expected code STATUS CODE`{}`, instead got `{}: {}`",
                    S_PARTIAL_CONTENT,
                    status_code,
                    response.status_msg()
                );
                return Ok(());
            }
            let range_header = response.header(H_CONTENT_RANGE);
            if range_header.is_none() {
                eprint!("Server did not provide a Content-Range header that's necessary for downloaded.");
                return Ok(());
            }
            let range_value = range_header.unwrap().value::<String>().unwrap();
            let tokens = range_value
                .replace("bytes ", "")
                .split_terminator(['-', '/'])
                .map(|str_value: &str| str_value.parse())
                .filter(|parse_result| parse_result.is_ok())
                .flatten()
                .collect::<Vec<usize>>();
            if tokens.len() < 3 {
                eprintln!("We received less than 3 elements in range. Currently not supported.");
                return Ok(());
            }

            let req_data = response.data();
            let body_length = req_data.len();
            out_file.write_all(req_data)?;
            total_written += body_length;
            println!("Downloaded: {} / {} ", total_written, tokens[2]);
            if total_written >= tokens[2] {
                print!("Done downloading");
                return Ok(());
            }
            let bytes_left = tokens[2] - total_written;
            chunk_size = std::cmp::min(MAX_CHUNK_SIZE, bytes_left);
            start_byte_index = tokens[1] + 1;

            request.put_header(
                H_RANGE,
                format!(
                    "bytes={}-{}",
                    start_byte_index,
                    start_byte_index + chunk_size
                ),
            );
        }
    }

    pub fn download(url: &HttpUrl) -> std::io::Result<()> {
        let file_size = Self::get_file_size(url)?;
        println!(
            "Downloading `{}` with size `{} bytes`",
            url.path(),
            file_size
        );
        if file_size > MAX_CHUNK_SIZE {
            Self::ranged_download(url, file_size)?;
        } else {
            Self::one_shot_download(url)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let url = HttpUrl::parse("192.168.1.8:8080/Video.mp4").expect("URL Parsing");
    Client::download(&url).unwrap();
    Ok(())
}
