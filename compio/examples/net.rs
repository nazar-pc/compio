use std::net::Ipv4Addr;

use compio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[compio::main(crate = "compio")]
async fn main() {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let addr = listener.local_addr().unwrap();

    let (mut tx, (mut rx, _)) =
        futures_util::try_join!(TcpStream::connect(&addr), listener.accept()).unwrap();

    tx.write_all("Hello world!").await.0.unwrap();

    let buffer = Vec::with_capacity(12);
    let (n, buffer) = rx.read_exact(buffer).await.unwrap();
    assert_eq!(n, buffer.len());
    println!("{}", String::from_utf8(buffer).unwrap());
}
