use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

async fn index_replacer() -> impl Responder {
    "Hello world!"
}

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // mutex for safety across threads
}

async fn counter_index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // get counter's MutexGuard
    *counter += 1;

    format!("Request number: {}", counter)
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // get app_name

    format!("Hello {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("A_Server_Name"),
            })
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index_replacer)),
            )
            // will serve to `GET /`
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
