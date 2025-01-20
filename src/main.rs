#[macro_use]
extern crate rocket;

use rocket::Config;
use std::net::Ipv4Addr;

pub mod payroll;

use crate::payroll::time_report::{time_report, time_reports};

#[get("/")]
fn index() -> &'static str {
    "Hello World! Hello Rocket!"
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 8080,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        temp_dir: "/tmp/config-example".into(),
        ..Config::debug_default()
    };

    rocket::build()
        .mount("/", routes![index, time_report, time_reports])
        .configure(config)
}
