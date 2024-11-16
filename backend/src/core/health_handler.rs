use actix_web::http::header;

#[actix_web::get("/health")]
async fn health() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
        .content_type(header::ContentType::plaintext())
        .body("Healthy")
}
