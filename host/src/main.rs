use host::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        println!("Connection established");
        pool.execute(|| handle_connection(stream));
    }

    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    const GET: &[u8] = b"GET / HTTP/1.1\r\n";
    const SLEEP: &[u8] = b"GET /sleep HTTP/1.1\r\n";


    let (status_line, file_name) = if buffer.starts_with(GET) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(SLEEP) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
