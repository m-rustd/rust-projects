use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs, thread, time::Duration};

use thread_pools::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "main.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    thread::sleep(Duration::from_secs(10));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on 127.0.0.1:7878");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(4) {
        let stream = stream?;
        // handle_client(stream);
        pool.execute(|| {
            handle_client(stream);
        });
    }
    Ok(())
}
