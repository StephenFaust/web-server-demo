use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

trait Codec {
    fn decode(socket: TcpStream) -> Vec<u8>;
    fn encode(data: &[u8]);
}

pub struct DefaultCodec;

impl DefaultCodec {
    pub async fn decode(socket: &mut TcpStream) -> Vec<u8> {
        let mut buf = [0; 8];
        let mut count: usize = 0;
        let mut length: usize = 0;
        match socket.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                } else {
                    length = usize::from_be_bytes(buf);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
        let mut buf_vec: Box<[u8]> = vec![0; length].into_boxed_slice();
        while count < length {
            match socket.read(&mut buf_vec[count..length]).await {
                Ok(n) => {
                    count += n;
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            }
        }
        return buf_vec.to_vec();
    }

    pub async fn encode( socket:&mut TcpStream, data: &[u8]) {
        let length = data.len();
        socket.write_u64(length as u64).await.unwrap();
        socket.write_all(data).await.unwrap();
        socket.flush().await.unwrap();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        tokio::task::spawn(async {});
    }
}
