use rocket::config::{Config, Environment};

#[macro_use]
#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

pub fn start() {
    let mut my_config = Config::new(Environment::Production);
    rocket::custom(my_config);
    rocket::ignite().mount("/", routes![hello]).launch();
}