mod user;
mod repository;
use std::{str::FromStr, sync::{Arc, atomic::{AtomicU16, Ordering}}};
use actix_web::{App, HttpServer, HttpResponse, web};
use repository::{MemoryRepository, Repository};


async fn get_user(user_id: web::Path<String>) -> HttpResponse {
    match uuid::Uuid::from_str(&user_id) {
        Ok(id) => {
            let repo = MemoryRepository::default();
            match repo.get_user(id) {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::NotFound().body("User not found"),
            }
        },
        Err(_) => HttpResponse::BadRequest().body("input user_id is not type for uuid::Uuid::from_str")
    }
 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    println!("Starting server");
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
    println!("Starting thread {}", thread_counter.fetch_add(1, Ordering::SeqCst));
    let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .route("/", web::get().to(|| HttpResponse::Ok().body("it is working")))
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route("/health", web::get().to(move || HttpResponse::Ok().header("thread-id", thread_index.to_string()).finish()))
    })
    .bind(address)
    .unwrap_or_else(|err| panic!("🔥🔥🔥 error starting server @ {} : {:?}", port, err))
    .run()
    .await
}


