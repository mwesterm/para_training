use crate::error_handler::ORMError;

use argon2::{password_hash::Error, Argon2, PasswordHash, PasswordVerifier};

async fn validate_credentials(
    user_name: &str,
    password: &str, // [...]
) -> Result<uuid::Uuid, ORMError> {
    let app_user = crate::db::models::AppUser::find_by_name(user_name).await?;
    /*
    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username."))
        .map_err(AuthError::InvalidCredentials)
        */
    Ok(uuid::Uuid::new_v4())
}

fn verify_password_hash(password: &str, hash: &str) -> Result<(), ORMError> {
    let argon2 = Argon2::default();
    let expected_password_hash = PasswordHash::new(hash).unwrap();

    match Argon2::default().verify_password(password.as_bytes(), &expected_password_hash) {
        Err(e) => {
            if e == argon2::password_hash::Error::Password {
                Err(ORMError::AuthInvalidCredentials)
            } else {
                Err(ORMError::AuthUnexpectedError)
            }
        }
        _ => Ok(()),
    }

    //.map_err(|e| AuthError::InvalidCredentials)
}
