mod api;
mod repository;
mod models;

#[macro_use] 
extern crate rocket;

use api::user_api::{create_user, get_user, get_all_users, update_user, delete_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    rocket::build().
        manage(MongoRepo::init())
            .mount("/", routes![create_user])
            .mount("/", routes![get_user])
            .mount("/", routes![get_all_users])
            .mount("/", routes![update_user])
            .mount("/", routes![delete_user])
}