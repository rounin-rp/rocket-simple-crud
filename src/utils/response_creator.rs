use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;


use crate::handler::{error_handler::TransmissionError, response::Response};


pub fn get_response(result: Result<String,TransmissionError>, success_message: &str)-> Result<status::Custom<Json<Response>>, status::Custom<Json<Response>>> {
    let response;
    match result {
        Ok(received_message) => {
            let message = Response::successful(format!("{} {}",success_message, received_message));
            response = Ok(status::Custom(Status::Accepted, Json(message)));
        },
        Err(error) => {
            match error {
                TransmissionError::RocketError(_) => {
                    let message = Response::unsuccessful(String::from("Rocket error"));
                    response = Err(status::Custom(Status::BadRequest, Json(message)));
                },
                TransmissionError::MongoError(_) => {
                    let message = Response::unsuccessful(String::from("Mongo db error"));
                    response = Err(status::Custom(Status::InternalServerError, Json(message)));
                },
                TransmissionError::Message(error) => {
                    let message = Response::unsuccessful(error);
                    response = Err(status::Custom(Status::InternalServerError, Json(message)));
                },
                TransmissionError::ValueError(_) => {
                    let message = Response::unsuccessful(String::from("Value not found"));
                    response = Err(status::Custom(Status::InternalServerError, Json(message)));
                }
            }
        }
    }
    response
}