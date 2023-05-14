use rocket::{self, get, routes};

#[get("/")]
fn welkom() -> &'static str {
    "Welkom op mijn site!!\n"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![welkom]).launch().await;
}
