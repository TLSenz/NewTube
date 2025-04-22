#[macro_use] extern crate rocket;



pub mod controller {
    pub mod usercontroller;
}

pub mod service{
    pub mod userservice;
}

pub mod model{
    pub mod usermodel;
}

use rocket::http::Status;
use crate::controller::usercontroller::login;
use crate::service::userservice::check_login;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}