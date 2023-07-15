use html::*;
fn main() {
    let body = body(
        "RustHTML",
        div(
            Some("wrapper"),
            Some(h1(
                Some("head"),
                "Fat",
                Some(h1(Some("head"),
                    "rat", 
                    Some(h1(Some("head"),a(Some("anchor"), "https://youtube.com", "youtube", Some("color: white;")).as_str(),None ,None )),
                Some("color: orange;"))),
                Some("color: red; background: black;"),
            )),
            Some(
                div(None, 
                    Some(img(Some("image"),"https://media.discordapp.net/attachments/1120124565591425034/1129330787980951572/image.png", Some("border-radius: 15px;") ,None ,None )),
                None, None)),
            // root styling for div
            Some("background: black; border-radius: 10px; margin: 10px; padding: 20px;"),
        ),
        Some("
            body {
                background-color: red;
            }
            h1 {
                background-color: white;
            }
            .wrapper {
                font-family: Arial, sans-serif;
            }
            .wrapper:hover {
                scale: 1.02;
            }
            .image {
                width: 100px;
            }
            .head {
                
            }
            .anchor {
                background-color: orange;
            }
            "),
        Some(script("document.body.style.backgroundColor = 'orange';"))
    );
    // let alt_body = external_body("./index.html");
    println!("{}", &body);
    // println!("{}", alt_body);
    HTML::start(body, "127.0.0.1", "8080");
}
