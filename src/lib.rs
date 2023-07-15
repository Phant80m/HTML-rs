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
    match cat {
        Ok(mut file) => {
            let mut file_contents = String::new();
            if let Err(error) = file.read_to_string(&mut file_contents) {
                eprintln!("Error reading file: {}", error);
            }

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
