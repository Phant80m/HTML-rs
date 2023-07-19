pub mod prelude;
pub mod rust_to_html;
use owo_colors::OwoColorize;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

pub trait Debug {
    fn debug(&self);
}

pub struct Server {
    host: String,
    port: u32,
}

pub struct ServerResponse {
    content: String,
}
pub struct StyleResponse {
    content: String,
}
pub struct Custom404 {
    content: String,
}
#[derive(Clone)]
pub struct CustomRoutes {
    pub path: String,
    pub content_type: String,
    pub response: String,
    pub styles: Option<String>,
}
impl Custom404 {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
    pub fn default() -> Self {
        Self {
            content: "404 page not found!".to_string(),
        }
    }
}

impl CustomRoutes {
    pub fn new(
        path: impl Into<String>,
        content_type: impl Into<String>,
        response: impl Into<String>,
        css_file: Option<impl Into<String>>,
    ) -> Self {
        Self {
            path: path.into(),
            content_type: content_type.into(),
            response: response.into(),
            styles: css_file.map(Into::into),
        }
    }

    pub fn include_css(&self) -> Option<String> {
        if let Some(styles) = &self.styles {
            match fs::read_to_string(styles) {
                Ok(css) => Some(css),
                Err(e) => {
                    eprintln!("Error reading CSS file: {}", e);
                    None
                }
            }
        } else {
            None
        }
    }
}
impl Server {
    pub fn new(host: impl Into<String>, port: u32) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn start(
        &self,
        content: ServerResponse,
        style: StyleResponse,
        custom_routes: Vec<CustomRoutes>,
        custom_404: Custom404,
    ) {
        let custom404 = custom_404.content.clone();
        let routes = custom_routes.clone();
        fn handle_client(
            mut stream: TcpStream,
            content: String,
            style: String,
            custom_routes: Vec<CustomRoutes>,
            custom_404: String,
        ) {
            let mut buffer = [0; 1024];
            let css_content = include_css(&style);
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let request = String::from_utf8_lossy(&buffer);

                    let response = if request.starts_with("GET / HTTP/1.1") {
                        // Handle the request for the root path (index.html)
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                            content
                        )
                    } else if request.starts_with("GET /style.css HTTP/1.1") {
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\n\r\n{}",
                            css_content
                        )
                    } else {
                        // Check for custom routes
                        for route in custom_routes.iter() {
                            if request.starts_with(&format!("GET {} HTTP/1.1", route.path)) {
                                let response = match &route.content_type[..] {
                                    "text/plain" => format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
                                        route.content_type, route.response
                                    ),
                                    "application/json" => format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
                                        route.content_type, route.response
                                    ),
                                    _ => format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                                        route.response
                                    ),
                                };
                                // Check if the custom route has a CSS file
                                if let Some(css_content) = &route.styles {
                                    // Inject the CSS content into the response using a <style> tag
                                    let styled_response = format!(
                                        "{}\r\n\r\n<style type=\"text/css\">\r\n{}\r\n</style>",
                                        response, css_content
                                    );
                                    return stream.write_all(styled_response.as_bytes()).unwrap();
                                } else {
                                    return stream.write_all(response.as_bytes()).unwrap();
                                }
                            }
                        }

                        // Handle other requests (404 Not Found)
                        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", custom_404)
                    };
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(e) => eprintln!("Error reading from connection: {}", e),
            }
        }

        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))
            .expect("Failed to bind adress!");
        println!("Server listening on http://{}:{}", self.host, self.port);

        thread::spawn({
            move || {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let content = content.content.clone();
                            let css_content = style.content.clone();
                            let custom_routes = custom_routes.clone();
                            let custom404 = custom404.clone();
                            thread::spawn(move || {
                                handle_client(
                                    stream,
                                    content,
                                    css_content,
                                    custom_routes,
                                    custom404,
                                );
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
                ".help" => {
                    println!("{}", "Commands:".green().bold());
                    println!("{}", ".stop - Stop the server".yellow());
                    println!("{}", ".info - Get server info".yellow());
                }
                ".stop" => {
                    println!("Shutting down");
                    std::process::exit(0);
                }
                ".routes" => {
                    for route in routes.iter() {
                        println!(
                            "{} - {}",
                            route.path.bold().green(),
                            route.content_type.bold().yellow()
                        );
                    }
                }
                ".info" => {
                    println!("Server listening on http://{}:{}", self.host, self.port);
                }
                _ => println!(
                    "{}{}",
                    "Unknown Command: ".red().bold(),
                    prompt.trim().red()
                ),
            }
        }
    }
}
fn include_css(style_path: &str) -> String {
    match fs::read_to_string(style_path) {
        Ok(css) => css,
        Err(e) => {
            eprintln!("Error reading CSS file: {}", e);
            "Error reading CSS file".to_string()
        }
    }
}

impl Debug for Server {
    fn debug(&self) {
        println!("Server {{ host: {}, port: {} }}", self.host, self.port)
    }
}
impl ServerResponse {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}
impl StyleResponse {
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
            eprintln!("Failed to read file {:?}", e);
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
