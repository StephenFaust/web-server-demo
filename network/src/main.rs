use std::{thread, time::Duration};

use tokio::net::{TcpListener, TcpStream};

pub mod codec;
pub mod chanel;

#[tokio::main]
async fn main() {
    tokio::task::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let (mut socket, _) = listener.accept().await.unwrap();
        loop {
            let data = codec::DefaultCodec::decode(&mut socket).await;
            let str = String::from_utf8(data).unwrap();
            println!("{}", str);
        }
    });
    thread::sleep(Duration::from_secs(1));
    tokio::task::spawn(async {
        // 建立 TCP 连接
        let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

        loop {
            // 发送数据到服务器
            thread::sleep(Duration::from_secs(2));
            let message = "Hello, server!";
            codec::DefaultCodec::encode(&mut socket, message.as_bytes()).await;
        }

        // // 从服务器接收数据
        // let mut buffer = [0u8; 1024];
        // let n = stream.read(&mut buffer).await?;
        // let received_message = String::from_utf8_lossy(&buffer[..n]);
    });

    loop {}
}
