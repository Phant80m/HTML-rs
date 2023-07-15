# HTML-rs
Simple rust library to write and server html directly from a program

## Example
```rust
fn main() {
    let body = body(
        /title
        "RustHTML",
        //body
        div(
            Some("Example-Class"),
            Some(h1(
                Some("Example-Class"),
                "Fat",
                Some(h1(Some("Example-Class"),
                    "rat",
                    Some(h1(Some("Example-Class"),a(Some("Example-Class"), "https://example.com", "example", Some("color: white;")).as_str(),None ,None )),
                Some("color: orange;"))),
                Some("color: red; background: black;"),
            )),
            Some(
                div(None,
                    Some(img(Some("image"),"example-image-src", Some("border-radius: 15px;") ,None ,None )),
                None, None)),
            
            None
        ),
        // stylesheet
        Some("
            body {
                background-color: red;
            }
            h1 {
                background-color: white;
            }
            
            .Example-Class {
            }
            "),
        Some(script("document.body.style.backgroundColor = 'orange';"))
    );
    // Serve the website with built in http server
    HTML::start(body, "127.0.0.1", "8080");
    // build the website
    HTML::build(body)
}

```
