use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    server_listener().await.unwrap();
}

async fn server_listener() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:29953").await?;

    println!("Listening to logs on port 29953");

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        println!("Received: {}", String::from_utf8_lossy(&buf[..len]));
    }
}