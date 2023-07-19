use libwizard::prelude::*;

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
            "/api",
            "application/json",
            "{\"name\": \"John\", \"age\": 30}",
            None::<String>,
        ),
    ];

    server.start(
        ServerResponse::new(include_html("./index.html")),
        StyleResponse::new("./style.css"),
        custom_routes,
        Custom404::new(include_html("./404.html")),
    );
}
