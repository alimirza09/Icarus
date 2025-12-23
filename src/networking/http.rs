use native_tls::TlsConnector as NativeTlsConnector;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsConnector;

async fn fetch_http(host: &str, port: u16) -> String {
    let mut stream = TcpStream::connect((host, port)).await.unwrap();
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        host
    );
    stream.write_all(request.as_bytes()).await.unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).to_string()
}

async fn fetch_https(host: &str, port: u16) -> String {
    let tcp = TcpStream::connect((host, port)).await.unwrap();
    let connector = TlsConnector::from(NativeTlsConnector::new().unwrap());
    let mut stream = connector.connect(host, tcp).await.unwrap();

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        host
    );
    stream.write_all(request.as_bytes()).await.unwrap();

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await.unwrap();
    String::from_utf8_lossy(&buf).to_string()
}

pub async fn connect_to_http_site(url: &str) -> String {
    let https = url.starts_with("https://");
    let host = url
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_end_matches('/');

    let port = if https { 443 } else { 80 };
    println!("Attempted to connect to {}:{}", host, port);

    let response = if https {
        fetch_https(host, port).await
    } else {
        fetch_http(host, port).await
    };

    remove_http_header(&response).to_string()
}

pub fn remove_http_header(reply: &str) -> &str {
    let body = reply
        .find("\r\n\r\n")
        .map(|pos| &reply[pos + 4..])
        .unwrap_or(reply);

    body.find("<!doctype html>")
        .map(|n| &body[n..])
        .unwrap_or(body)
}
