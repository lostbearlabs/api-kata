use std::collections::HashMap;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::web::Query;

// SEE: https://actix.rs/docs/getting-started

#[get("/api/v1/hello")]
async fn hello(req: HttpRequest) -> impl Responder {

    let params = Query::<HashMap<String, String>>::from_query(req.query_string())
        .unwrap();
    let default_name = &String::from("world");
    let name = params.get("name").unwrap_or(default_name);

    let response_text = format!("Hello, {}!", name);
    HttpResponse::Ok().body(response_text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
