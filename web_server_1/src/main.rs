use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000");

  if let Err(e) = listener {
    eprintln!("Err: failed to bind socket to the address; {}", e);
    return
  }

  let listener = listener.unwrap();
  println!("Server is listening...");
  
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        thread::spawn(move || {
          let mut stream = stream;
          let mut buf = [0; 1024];
          
          if let Err(e) = stream.read(&mut buf) {
            eprintln!("Err: failed to read data from the socket; {}", e);
            return
          };

          if let Err(e) = stream.write(b"HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: 11\n\nhello world") {
            eprintln!("Err: failed to write data to the socket; {}", e);
            return;
          };
        });
      }
      Err(e) => {
        eprintln!("Err: failed to handle incoming connection; {}", e);
      }
    }
  }
}
