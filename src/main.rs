use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:27005").await?;
    
    let mut buf = [0u8; 1024];

    println!("Listening to logs on port 27005");

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        println!("Received: {}", String::from_utf8_lossy(&buf[..len]));

        let len = socket.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
}