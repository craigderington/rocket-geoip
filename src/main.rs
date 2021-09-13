#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::request::Form;
use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(FromForm)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/login", data = "<user_form>")]
fn login(user_form: Form<UserLogin>) -> String {
   format!("Hello, {}!", user_form.username)
}

#[get("/geolocate/<ipaddress>")]
fn geolocate(ipaddress: String) -> String {
    let reader = maxminddb::Reader::open_readfile("data/GeoLite2-City.mmdb").unwrap();
    let ip: IpAddr = FromStr::from_str(&ipaddress).unwrap();
    let city: geoip2::City = reader.lookup(ip).unwrap();
    format!("Your city is: {:?}", city)
}

fn main() {
    rocket::ignite().mount("/", 
    routes![index, hello, login, geolocate]
    ).launch();
}
