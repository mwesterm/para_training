use actix_web::{dev::ServiceRequest, Error};

pub async fn auth_extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    Ok(vec!["ADMIN".to_string()])
}
