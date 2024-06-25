use std::{time::Duration, sync::atomic::{AtomicUsize, Ordering}};
use actix_web::{Responder, get, HttpServer, web, App, rt::time::sleep};

const EXPLANATION: &str = "Usage:";

static COUNTER: AtomicUsize = AtomicUsize::new(1);

#[get("/{delay}/{message}")]
async fn delay(path: web::Path<(u64, String)>) -> impl Responder {
    let (delay_ms, message) = path.into_inner();
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);
    print!("#{count} - {delay_ms}ms:{message}");
    sleep(Duration::from_millis(delay_ms)).await;
    message
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{EXPLANATION}");
    HttpServer::new(||{
        App::new().service(delay)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

