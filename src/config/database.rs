use mongodb::Client;
use dotenv;
use crate::handler::error_handler::TransmissionError;

pub struct DatabaseClient {
    pub client: Client
}

impl DatabaseClient {
    pub async fn connect() -> Result<Self, TransmissionError> {
        dotenv::dotenv().ok();
        let db_address = std::env::var("MONGO_CLIENT").expect("MONGO_CLIENT not set");
        let db_address = db_address.as_str();
        let client = Client::with_uri_str(db_address).await?;
        Ok(
            Self{
                client: client
            }
        )
    }
}


// pub async fn connect_to_mongo() -> Result<Client, TransmissionError>{
//     dotenv::dotenv().ok();
//     let db_address = std::env::var("MONGO_CLIENT").expect("MONGO_CLIENT not set");
//     let db_address = db_address.as_str();
//     let client = Client::with_uri_str(db_address).await?;
//     Ok(client)
// }