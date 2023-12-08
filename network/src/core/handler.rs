pub struct NetHandler;

impl NetHandler {
    pub fn on_message(data: Vec<u8>) {
        let str = String::from_utf8(data).unwrap();
        println!("on_message: {}", str);
    }
    pub fn on_open() {
        println!("on_open");
    }
    pub fn on_close() {
        println!("on_close");
    }
    pub fn on_error() {
        println!("on_error");
    }
}
