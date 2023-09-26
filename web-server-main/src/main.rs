use std::{thread, time::Duration};

use thread_pool::ThreadPool;



fn main() {
    let mut thread_pool = ThreadPool::new(10);
    for i in 1..100 {
        // thread::sleep(Duration::from_secs(1));
        thread_pool
            .execute(move || {
                println!("current thread {:?} ,num: {}", thread::current().id(), i);
                thread::sleep(Duration::from_secs(10));
            })
            .unwrap();
    }

    loop {}

    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     thread::spawn(|| handle_connection(stream));
    // }

    // fn handle_connection(mut stream: TcpStream) {
    //     let mut buffer = [0; 1024];
    //     stream.read(&mut buffer).unwrap();
    //     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //     let contents = fs::read_to_string("hello.html").unwrap_or_else(|err| {
    //         println!("{}", err);
    //         String::from("value")
    //     });
    //     let thread_id =
    //     println!("current threadId is {:?}", thread_id);
    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
}
