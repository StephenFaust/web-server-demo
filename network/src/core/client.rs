use std::sync::Arc;

use tokio::{
    net::TcpStream,
    runtime::Runtime,
    select,
    sync::mpsc::{self, Receiver, Sender},
};

use super::{codec, handler::NetHandler};

pub struct Client<'a> {
    pub addr: &'a str,
    recv: Option<Receiver<Vec<u8>>>,
    send: Sender<Vec<u8>>,
    rt_worker: Arc<Runtime>,
}
impl<'a> Client<'a> {
    pub fn new(addr: &'a str) -> Client<'a> {
        let (tx, rx) = mpsc::channel(1024);
        let rt_worker = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .max_io_events_per_tick(1024)
                .worker_threads(num_cpus::get() << 1)
                .build()
                .unwrap(),
        );
        return Client {
            addr,
            recv: Some(rx),
            send: tx,
            rt_worker,
        };
    }

    pub async fn connect(&mut self) {
        let mut socket: TcpStream = TcpStream::connect(self.addr).await.unwrap();
        let mut rx = self.recv.take().unwrap();
        self.rt_worker.spawn(async move {
            let (mut reader, mut writer) = socket.split();
            loop {
                select! {
                    send_info = rx.recv() => {
                        if let Some(send_info) = send_info {
                            codec::DefaultCodec::encode(&mut writer,&send_info).await;
                        }
                    },
                    data = codec::DefaultCodec::decode(&mut reader) => {
                        NetHandler::on_message(data);
                     },
                }
            }
        });
    }
    // self.writer = Some(Arc::new(Mutex::new(writer)));

    pub async fn send(&self, data: Vec<u8>) {
        let sx = self.send.clone();
        let _ = sx.send(data).await;
    }
}
