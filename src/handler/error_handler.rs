use mongodb;
use mongodb::bson::document::ValueAccessError;

#[derive(Debug)]
pub enum TransmissionError {
    RocketError(rocket::Error),
    MongoError(mongodb::error::Error),
    Message(String),
    ValueError(ValueAccessError)
}

impl From<rocket::Error> for TransmissionError {
    fn from(error: rocket::Error) -> Self {
        Self::RocketError(error)
    }
}

impl From<String> for TransmissionError {
    fn from(error: String) -> Self {
        Self::Message(error)
    }
}

impl From<mongodb::error::Error> for TransmissionError {
    fn from(error: mongodb::error::Error) -> Self {
        Self::MongoError(error)
    }
}

impl From<ValueAccessError> for TransmissionError {
    fn from(error: ValueAccessError) -> Self {
        Self::ValueError(error)
    }
}
