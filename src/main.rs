#![feature(proc_macro_hygiene, decl_macro)]

mod tests;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Rocket"
}


#[get("/hi")]
fn hi() -> &'static str {
    "Hi Rocket, updating!"
}

#[get("/hi/<name>")]
fn name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/v1", routes![index, hi, name])
}
