use dotenv;
use mongodb::bson::{doc, Document};

use crate::config::database::DatabaseClient;
use crate::handler::error_handler::TransmissionError;
use crate::models::user::User;


pub async fn create_user(username:String, password:String) -> Result<String, TransmissionError> {
    dotenv::dotenv().ok();
    let database_name = std::env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not set");
    let database_name = database_name.as_str();
    let collection_name = std::env::var("COLLECTION_NAME").expect("COLLECTION_NAME is not set");
    let collection_name = collection_name.as_str();
    let db_client = DatabaseClient::connect().await?;
    let database = db_client.client.database(database_name);
    let user_collection = database.collection::<Document>(collection_name);
    user_collection.insert_one(doc!{
        "username": username,
        "password": password
    }, None).await?;
    Ok(String::from("Success"))
}

pub async fn read_user(username: String) -> Result<String, TransmissionError> {
    dotenv::dotenv().ok();
    let database_name = std::env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not set");
    let database_name = database_name.as_str();
    let collection_name = std::env::var("COLLECTION_NAME").expect("COLLECTION_NAME is not set");
    let collection_name = collection_name.as_str();
    let db_client = DatabaseClient::connect().await?;
    let database = db_client.client.database(database_name);
    let user_collection = database.collection::<Document>(collection_name);
    let query = doc!{
        "username": username
    };
    let password;
    if let Some(user_doc) = user_collection.find_one(query, None).await? {
        password = user_doc.get_str("password")?;
        return Ok(String::from(password));
    }else{
        return Err(TransmissionError::Message(String::from("Document not found in database")));
    }
}

pub async fn delete_user(username: String) -> Result<String, TransmissionError> {
    dotenv::dotenv().ok();
    let database_name = std::env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not set");
    let database_name = database_name.as_str();
    let collection_name = std::env::var("COLLECTION_NAME").expect("COLLECTION_NAME is not set");
    let collection_name = collection_name.as_str();
    let db_client = DatabaseClient::connect().await?;
    let database = db_client.client.database(database_name);
    let user_collection = database.collection::<Document>(collection_name);
    let query = doc!{
        "username": username
    };
    user_collection.find_one_and_delete(query, None).await?;
    Ok(String::from("Success"))
}

pub async fn update_user(username: String, user: User) -> Result<String, TransmissionError> {
    dotenv::dotenv().ok();
    let database_name = std::env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not set");
    let database_name = database_name.as_str();
    let collection_name = std::env::var("COLLECTION_NAME").expect("COLLECTION_NAME is not set");
    let collection_name = collection_name.as_str();
    let db_client = DatabaseClient::connect().await?;
    let database = db_client.client.database(database_name);
    let user_collection = database.collection::<Document>(collection_name);
    let query = doc!{
        "username": username
    };
    let replace = doc! {
        "username" : user.username,
        "password" : user.password
    };
    user_collection.find_one_and_update(query, replace, None).await?;
    Ok(String::from("Success"))
}