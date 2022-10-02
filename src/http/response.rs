use super::StatusCode;


#[derive(Debug)]
pub struct Responce{
    status_code: StatusCode,
    body: Option<String>,
}

impl Responce {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Responce{status_code, body}
    }
    
}