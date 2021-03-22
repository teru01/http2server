use anyhow::Result;
use bytes::Bytes;
use h2::{server, RecvStream};
use http::{Request, Response};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:50000").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handler(stream).await {
                dbg!(e);
            }
        });
    }
}

async fn handler(stream: TcpStream) -> Result<()> {
    println!("incoming connection from {}", stream.peer_addr()?);
    let mut connection = server::handshake(stream).await?;

    while let Some(result) = connection.accept().await {
        let (request, mut respond) = result?;
        let resp_body = create_response_body(request)?;
        let response = Response::new(());

        let mut send = respond.send_response(response, false)?;
        send.send_data(Bytes::from(resp_body), true)?;
    }
    Ok(())
}

fn create_response_body(request: Request<RecvStream>) -> Result<Vec<u8>> {
    let file = File::open(format!("./contents{}", request.uri().path()))?;
    let mut file_reader = BufReader::new(file);
    let mut resp_body = Vec::new();
    file_reader.read_to_end(&mut resp_body)?;
    Ok(resp_body)
}
