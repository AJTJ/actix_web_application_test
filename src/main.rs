use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
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

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/file")
            .route(web::get().to(|| HttpResponse::Ok().body("reached the scoped config")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    // let user_scope = web::scope("/users").service(show_users);

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("A_Server_Name"),
            })
            .app_data(counter.clone())
            .route("/hit", web::get().to(counter_index))
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("/index.html", web::get().to(index_replacer)),
            )
            // will serve to `GET /`
            .service(index)
            .service(web::scope("/scoped").configure(scoped_config))
    })
    // bind must be used to bind to a specific socket address
    .bind("127.0.0.1:8080")?
    // run returns an instance of the `Server` type
    .run()
    .await
}
