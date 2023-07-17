pub mod html_to_rs;
use std::collections::linked_list;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    host: String,
    port: u32,
}

pub struct ServerResponse {
    content: String,
}

impl Server {
    pub fn new(host: impl Into<String>, port: u32) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn start(&self, content: ServerResponse) {
        fn handle_client(mut stream: TcpStream, content: String) {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(e) => println!("Error reading from connection: {}", e),
            }
        }

        let listener = Arc::new(Mutex::new(
            TcpListener::bind(format!("{}:{}", self.host, self.port))
                .expect("Failed to bind to address"),
        ));
        println!("Server listening on http://{}:{}", self.host, self.port);

        let server_handler = thread::spawn({
            let listener = listener.clone();
            move || {
                for stream in listener.lock().unwrap().incoming() {
                    match stream {
                        Ok(stream) => {
                            let content = content.content.clone();
                            // Spawn a new thread to handle the client
                            thread::spawn(move || {
                                handle_client(stream, content.clone());
                            });
                        }
                        Err(e) => {
                            println!("Error accepting connection: {}", e);
                        }
                    }
                }
            }
        });
        loop {
            let mut prompt = String::new();
            std::io::stdin().read_line(&mut prompt).unwrap_or_default();

            match prompt.trim() {
                ".stop" => {
                    println!("Shutting down");
                    std::process::exit(0);
                }
                _ => println!(""),
            }
        }
    }
}

impl ServerResponse {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}
pub fn include_html(path: impl Into<String> + std::convert::AsRef<std::path::Path>) -> String {
    let binding = path.into();
    let pth = Path::new(&binding);
    let cat = File::open(pth);
    match cat {
        Ok(mut file) => {
            let mut file_contents = String::new();
            if let Err(error) = file.read_to_string(&mut file_contents) {
                eprintln!("Error reading file: {}", error);
            }

            return file_contents;
        }
        Err(e) => {
            return format!(
                "
                <div style='text-align: center; font-family: sans-serif;'>
                    <h1 style='margin: auto; background: red; border-radius: 5px; padding: 5px; color: white;'>Error reading your file!</h1>
                    <h2>Cannot find {}<br></h2>
                    <p>{}</p>
                </div>",
                &binding, e,
            );
        }
    }
}
