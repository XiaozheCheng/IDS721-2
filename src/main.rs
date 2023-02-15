use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;
use rand::thread_rng;

async fn generate_password() -> impl Responder {
    // Define a list of possible characters for the password
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789)(*&^%$#@!~";
    
    // Generate a password of length 12, using the above character set
    let password: String = (0..12)
        .map(|_| {
            let mut rng = thread_rng();
            *charset.choose(&mut rng).unwrap() as char
        })
        .collect();

    // Return the generated password as a response
    HttpResponse::Ok().body(password)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/password", web::get().to(generate_password))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
