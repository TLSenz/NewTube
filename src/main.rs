#[macro_use] extern crate rocket;

use dotenv::dotenv;

pub mod controller {
    pub mod usercontroller;
}

pub mod service{
    pub mod userservice;
}

pub mod model{
    pub mod usermodel;
}

pub mod security{
    pub mod authentication;
}

use rocket::http::Status;
use crate::controller::usercontroller::login;
use crate::service::userservice::check_login;



#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![login])
}