#[macro_use] extern crate rocket;



use rocket::serde::json::Json;
use rocket::response::status;


mod models;
mod handler; 
mod controller;
mod config;
mod utils;

use crate::models::user::User; 
use crate::handler::response::Response;
use crate::controller::operations;
use crate::utils::response_creator::get_response;



#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[post("/create-user", data = "<user>")]
async fn create_user(user: Json<User>)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>>{
    let u = user.into_inner();
    // println!("The username got is {}",u.username);
    let result = operations::create_user(u.username, u.password).await;
    let response = get_response(result,"User creation");
    response
}

#[get("/get-user/<username>")]
async fn get_user(username: String)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>> {
    let result = operations::read_user(username).await;
    let response = get_response(result,"The password of user is");
    response
}

#[delete("/delete-user", data = "<user>")]
async fn delete_user(user: Json<User>)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>>{
    let username = user.username.clone();
    let result = operations::delete_user(username).await;
    let response = get_response(result, "User deletion");
    response
}

#[patch("/update-user", data = "<user>")]
async fn update_user(user: Json<User>)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>> {
    let u = user.into_inner();
    let result = operations::update_user(u.username.clone(), u).await;
    let response = get_response(result, "User updation");
    response
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_user, get_user, delete_user, update_user])
}