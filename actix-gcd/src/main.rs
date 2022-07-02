use std::io;
use actix_web::{web, App, HttpServer};



use actix_gcd::responses;



#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = 3000;
    let address = "127.0.0.1";
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(responses::get_index))
            .service(responses::post_gcd)
    });

    println!("Serving on http://localhost:{port}");

    server
        .bind(format!("{address}:{port}"))?
        .run()
        .await
}
