use compio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};

#[compio_macros::test]
async fn accept_read_write() -> std::io::Result<()> {
    let dir = tempfile::Builder::new()
        .prefix("compio-uds-tests")
        .tempdir()
        .unwrap();
    let sock_path = dir.path().join("connect.sock");

    let listener = UnixListener::bind(&sock_path)?;

    let mut client = UnixStream::connect(&sock_path)?;
    let (mut server, _) = listener.accept().await?;

    let write_len = client.write_all("hello").await.0?;
    assert_eq!(write_len, 5);
    drop(client);

    let buf = Vec::with_capacity(5);
    let (res, buf) = server.read_exact(buf).await.unwrap();
    assert_eq!(res, 5);
    assert_eq!(&buf[..], b"hello");
    let len = server.read(buf).await.0?;
    assert_eq!(len, 0);
    Ok(())
}

#[compio_macros::test]
async fn shutdown() -> std::io::Result<()> {
    let dir = tempfile::Builder::new()
        .prefix("compio-uds-tests")
        .tempdir()
        .unwrap();
    let sock_path = dir.path().join("connect.sock");

    let listener = UnixListener::bind(&sock_path)?;

    let mut client = UnixStream::connect(&sock_path)?;
    let (mut server, _) = listener.accept().await?;

    // Shut down the client
    client.shutdown().await?;
    // Read from the server should return 0 to indicate the channel has been closed.
    let n = server.read(Vec::with_capacity(1)).await.0?;
    assert_eq!(n, 0);
    Ok(())
}
