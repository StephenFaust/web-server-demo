use std::{thread, time::Duration};

use network::core::{client::Client, server::Server};
use tokio::signal;

#[tokio::main]
async fn main() {
    let server = Server::new("192.168.30.23:8080");
    server.start().await;
    // thread::sleep(Duration::from_secs(2));
    // let rt_worker = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .max_io_events_per_tick(1024)
    //     .worker_threads(10)
    //     .build()
    //     .unwrap();

    // let mut client = Client::new("127.0.0.1:8080");

    // client.connect().await;

    // rt_worker.spawn(async move {
    //     let mut i = 0;
    //     loop {
    //         let data = format!("client hello {}", i).as_bytes().to_vec();

    //         let _ = client.send(data).await;

    //         thread::sleep(Duration::from_secs(1));
    //         println!("send.......");
    //         i += 1;
    //         // tokio::task::yield_now().await;
    //     }
    // });

    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);
    tokio::select! {
            _ = ctrl_c => {
                std::process::exit(0);
            }
    }
}
