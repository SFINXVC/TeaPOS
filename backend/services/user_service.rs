use diesel_async::AsyncPgConnection;

use crate::models::user::{NewUser, User};
use crate::errors::{Error, Result, AuthError};

pub async fn register_user(conn: &mut AsyncPgConnection, user: NewUser) -> Result<User> {
    let user = User::create(user, conn).await?;

    Ok(user)
}

pub async fn login_user(conn: &mut AsyncPgConnection, username: &str, password: &str) -> Result<User> {
    let user = User::find_by_username(username, conn).await?;
    
    // Verify the password using Argon2id
    if !User::verify_password(&user.password, password)? {
        return Err(Error::Auth(AuthError::InvalidCredentials));
    }
    
    Ok(user)
}