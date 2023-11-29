use rocket::Rocket;


#[rocket::launch]
async fn launch() -> _ {
    Rocket::build()
}
