use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>
}

impl Response {
    pub fn successful(message: String) -> Self {
        Self{
            success: true,
            message: Some(message),
            error: None
        }
    }

    pub fn unsuccessful(error: String) -> Self {
        Self{
            success: false,
            message: None,
            error: Some(error)
        }
    }
}

