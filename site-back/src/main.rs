use rocket::{Rocket, get, routes};

#[get("/index.html")]
fn index() -> &'static str {
    "Hello, World!"
}

#[rocket::launch]
async fn launch() -> _ {
    Rocket::build()
        .configure(rocket::Config {
            port: 9115,
            ..Default::default()
        })
        .mount("/", routes![ index ])
}
