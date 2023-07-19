use libwizard::{include_html, Server, ServerResponse, StyleResponse, CustomRoutes};

fn main() {
    let server = Server::new("127.0.0.1", 8080);
    let custom_routes = vec![
        CustomRoutes::new(
            "/about",
            "text/html",
            include_html("./about.html"),
            Some("./about.css"),
        ),
        CustomRoutes::new(
            "/api/main",
            "application/json",
            "Hello",
            Some(""))
    ];
    
    
    let response = ServerResponse::new(include_html("./index.html"));
    let style = StyleResponse::new("./style.css");
    server.start(response, style, custom_routes);
   
}
