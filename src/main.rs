use libwizard::{include_html, Server, ServerResponse};

fn main() {
    let server = Server::new("127.0.0.1", 8080);

    let response = ServerResponse::new(include_html("./index.html"));
    server.start(response);
}
