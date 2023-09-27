use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use thread_pool::ThreadPool;

fn main() {
    let mut thread_pool = ThreadPool::new(10);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| handle_connection(stream)).unwrap();
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        let contents = fs::read_to_string("hello.html").unwrap_or_else(|err| {
            println!("{}", err);
            String::from("value")
        });
        println!("current threadId is {:?}", thread::current().id());
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
