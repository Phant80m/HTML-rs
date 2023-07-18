<<<<<<< HEAD
use minify_html;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

pub struct HTML;

impl HTML {
    pub fn build(content: &String) {
        fn export(content: String) -> std::io::Result<()> {
            let dest = Path::new("./dest");
            if dest.exists() {
                println!("Overriding old release");
                if fs::remove_dir_all(dest).is_err() {
                    eprintln!("Error removing destination");
                }
            }
            fs::create_dir(dest)?;

            let path = "./dest/index.html";
            let mut output_file = File::create(path)?;

            let cfg = minify_html::Cfg::new();
            cfg.minify_css;
            cfg.keep_closing_tags;

            let code: &[u8] = content.as_bytes();
            let compacted = minify_html::minify(code, &cfg);
            write!(
                output_file,
                "{}",
                String::from_utf8(compacted).expect("error")
            )?;
            println!("html Built!");
            Ok(())
        }

        if let Err(e) = export(content.to_string()) {
            eprintln!("{:?}", e);
        }
    }
    fn handle_request(mut stream: TcpStream, content: String) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            content
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    pub fn start(content: String, bind: &str, port: &str) {
        let listener = TcpListener::bind(format!("{}:{}", bind, port)).unwrap();
        println!("Server listening on http://{}:{}!", bind, port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let content = content.clone();
            thread::spawn(move || {
                HTML::handle_request(stream, content.clone());
            });
        }
    }
}
pub fn external_body(path: &str) -> String {
    let cat = File::open(path);
=======
pub mod rust_to_html;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};
use owo_colors::OwoColorize;

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
#[derive(Clone)]
pub struct CustomRoutes {
    pub path: String,
    pub content_type: String,
    pub response: String,
    pub styles: Option<String>
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
            // Load the CSS from the file and return it
            match fs::read_to_string(styles) {
                Ok(css) => Some(css),
                Err(e) => {
                    eprintln!("Error reading CSS file: {}", e);
                    None
                }
            }
        } else {
            // No CSS file specified, return None
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

    pub fn start(&self, content: ServerResponse, style: StyleResponse, custom_routes: Vec<CustomRoutes>,) {
        let routes = custom_routes.clone();
        fn handle_client(mut stream: TcpStream, content: String, style: String, custom_routes: Vec<CustomRoutes>,) {
            let mut buffer = [0; 1024];
            let css_content = load_css_from_file(&style);
            match stream.read(&mut buffer) {
                Ok(_) => {
                    let request = String::from_utf8_lossy(&buffer);
                  
                    let response = if request.starts_with("GET / HTTP/1.1") {
                        // Handle the request for the root path (index.html)
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", content)
                    } else if request.starts_with("GET /style.css HTTP/1.1") {
                        format!("HTTP/1.1 200 OK\r\nContent-Type: text/css\r\n\r\n{}", css_content)
                    } else {
                        // Check for custom routes
                        for route in custom_routes.iter() {
                    if request.starts_with(&format!("GET {} HTTP/1.1", route.path)) {
                        let mut response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
                            route.content_type, route.response
                        );

                        // Check if the custom route has a CSS file
                        if let Some(css_content) = route.include_css() {
                            // Inject the CSS content into the response using a <style> tag
                            response = format!(
                                "{}\r\n\r\n<style type=\"text/css\">\r\n{}\r\n</style>",
                                response, css_content
                            );
                        }

                        return stream.write_all(response.as_bytes()).unwrap();
                    }
                }
         
                
                        // Handle other requests (404 Not Found)
                        format!("HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found")
                    };
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(e) => eprintln!("Error reading from connection: {}", e),
            }
        }

        let listener = 
            TcpListener::bind(format!("{}:{}", self.host, self.port)).expect("Failed to bind to address");
        println!("Server listening on http://{}:{}", self.host, self.port);

        thread::spawn({
            move || {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            let content = content.content.clone();
                            let css_content = style.content.clone();
                            let custom_routes = custom_routes.clone();
                            thread::spawn(move || {
                                handle_client(stream, content, css_content, custom_routes);
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
                        println!("{} - {}", route.path.bold().green(), route.content_type.bold().yellow());
                    }
                }
                ".info" => {
                    println!("Server listening on http://{}:{}", self.host, self.port);
                }
                _ => println!("{}{}", "Unknown Command: ".red().bold(), prompt.trim().red())
            }
        }
    }
}
fn load_css_from_file(style_path: &str) -> String {
    match fs::read_to_string(style_path) {
        Ok(css) => css,
        Err(e) => {
            eprintln!("Error reading CSS file: {}", e);
            "/* Error reading CSS file */".to_string()
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
>>>>>>> master
    match cat {
        Ok(mut file) => {
            let mut file_contents = String::new();
            if let Err(error) = file.read_to_string(&mut file_contents) {
                eprintln!("Error reading file: {}", error);
            }

<<<<<<< HEAD
            // Print the file contents
            return file_contents;
        }
        Err(e) => {
            eprintln!("Failed to read file {:?}", e);
            return String::from("Error!");
        }
    }
}
pub fn body(title: &str, child: String, style: Option<&str>, script: Option<String>) -> String {
    format!(
        "{}{}{}{} {} {} {} {}{}{}\n{}{}",
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=, initial-scale=">
"#,
        "    <title>",
        title,
        "</title>",
        "\n<style>\n",
        style.unwrap_or_default(),
        "\n</style>",
        "
</head>
<body>",
        child,
        "
        <script>",
        script.unwrap_or_default(),
        "</script>
</body>
</html>
"
    )
}
pub fn h1(class: Option<&str>, value: &str, child: Option<String>, style: Option<&str>) -> String {
    format!(
        "\n    <h1 class='{}' style='{}'>{}{}</h1>",
        class.unwrap_or_default(),
        style.unwrap_or_default(),
        value,
        child.unwrap_or_default()
    )
}
pub fn h2(class: Option<&str>, value: &str, child: Option<String>, style: Option<&str>) -> String {
    format!(
        "\n    <h2 class='{}' style='{}'>{}{}</h2>",
        class.unwrap_or_default(),
        style.unwrap_or_default(),
        value,
        child.unwrap_or_default()
    )
}
pub fn h3(class: Option<&str>, value: &str, child: Option<String>, style: Option<&str>) -> String {
    format!(
        "\n    <h3 class='{}' style='{}'>{}{}</h3>",
        class.unwrap_or_default(),
        style.unwrap_or_default(),
        value,
        child.unwrap_or_default()
    )
}
pub fn div(
    class: Option<&str>,
    child: Option<String>,
    child2: Option<String>,
    style: Option<&str>,
) -> String {
    format!(
        "\n    <div class='{}' style='{}'>{}\n    {}\n    </div>",
        class.unwrap_or_default(),
        style.unwrap_or_default(),
        child.unwrap_or_default(),
        child2.unwrap_or_default()
    )
}

pub fn img(
    class: Option<&str>,
    src: &str,
    style: Option<&str>,
    width: Option<&str>,
    height: Option<&str>,
) -> String {
    let w: Option<String> = Some(format!("width={}", width.unwrap_or("auto")));
    let h: Option<String> = Some(format!("height={}", height.unwrap_or("auto")));
    format!(
        "\n    <img class='{}' src='{}' style='{}', {}, {}",
        class.unwrap_or_default(),
        src,
        style.unwrap_or_default(),
        w.unwrap_or_default(),
        h.unwrap_or_default()
    )
}
pub fn a(class: Option<&str>, href: &str, value: &str, style: Option<&str>) -> String {
    format!(
        "\n    <a class='{}' href='{}', style='{}'>{}</a>",
        class.unwrap_or_default(),
        href,
        style.unwrap_or_default(),
        value
    )
}
pub fn script(script: &str) -> String {
    script.to_string()
}
=======
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
>>>>>>> master
