use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn collect_system_info(req: String) -> impl Responder {
    println!("Received system information:");
    println!("{}", req);

    HttpResponse::Ok().body("System information received successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/collect").to(collect_system_info))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
