use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::AsyncPgConnection;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
use diesel::{AsExpression, FromSqlRow};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::result::Error as DieselError;
use argon2::{password_hash::{SaltString, rand_core::OsRng}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::error::{Error as AppError, Result};
use thiserror::Error;

use crate::schema::users;
use crate::schema::sql_types::UserRole as UserRoleSqlType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = UserRoleSqlType)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    User,
    Employee,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "superadmin"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Employee => write!(f, "employee"),
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "superadmin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "employee" => Ok(UserRole::Employee),
            _ => Err(format!("Unknown user role: {}", s)),
        }
    }
}

impl ToSql<UserRoleSqlType, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            UserRole::SuperAdmin => <&str as ToSql<Text, Pg>>::to_sql(&"superadmin", out),
            UserRole::Admin => <&str as ToSql<Text, Pg>>::to_sql(&"admin", out),
            UserRole::User => <&str as ToSql<Text, Pg>>::to_sql(&"user", out),
            UserRole::Employee => <&str as ToSql<Text, Pg>>::to_sql(&"employee", out)
        }
    }
}


impl FromSql<UserRoleSqlType, Pg> for UserRole {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        let s = std::str::from_utf8(bytes.as_bytes())?;
        match s {
            "superadmin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "employee" => Ok(UserRole::Employee),
            s => Err(format!("Unrecognized enum variant: {}", s).into()),
        }
    }
}

impl From<UserError> for AppError {
    fn from(error: UserError) -> Self {
        AppError::DatabaseError(error.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub whatsapp: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub fullname: String,
    pub password: String,
    pub whatsapp: String,
    pub role: UserRole,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User with username '{0}' already exists")]
    UserAlreadyExists(String),

    #[error("User with ID '{0}' not found")]
    UserIDNotFound(i64),

    #[error("User with username '{0}' not found")]
    UsernameNotFound(String),

    #[error("Unexpected database error: {0}")]
    DatabaseError(#[from] DieselError),
}

impl User {
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| UserError::InvalidCredentials.into())
    }
    
    pub fn verify_password(hash: &str, password: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| UserError::InvalidCredentials)?;
            
        Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    pub async fn create(mut new_user: NewUser, conn: &mut AsyncPgConnection) -> Result<User> {
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .await;
            
        if let Ok(_) = existing_user {
            return Err(UserError::UserAlreadyExists(new_user.username).into());
        }
        
        if !Self::is_hashed_password(&new_user.password) {
            new_user.password = Self::hash_password(&new_user.password)?;
        }
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e).into())
    }

    pub async fn create_and_return(mut new_user: NewUser, conn: &mut AsyncPgConnection) -> Result<User> {
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .await;

        if let Ok(_) = existing_user {
            return Err(UserError::UserAlreadyExists(new_user.username).into());
        }
        
        // hash the password if it hasn't been hashed yet.
        if !Self::is_hashed_password(&new_user.password) {
            new_user.password = Self::hash_password(&new_user.password)?;
        }

        let created_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;

        Ok(created_user)
    }

    pub async fn find_by_id(id: i64, conn: &mut AsyncPgConnection) -> Result<User> {
        users::table
            .find(id)
            .first(conn)
            .await
            .map_err(|e| {
                if let DieselError::NotFound = e {
                    UserError::UserIDNotFound(id).into()
                } else {
                    UserError::DatabaseError(e).into()
                }
            })
    }

    pub async fn find_by_username(username: &str, conn: &mut AsyncPgConnection) -> Result<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
            .await
            .map_err(|e| {
                if let DieselError::NotFound = e {
                    UserError::UsernameNotFound(username.to_string()).into()
                } else {
                    UserError::DatabaseError(e).into()
                }
            })
    }

    pub async fn update(&self, conn: &mut AsyncPgConnection) -> Result<User> {
        diesel::update(users::table.find(self.id))
            .set(self)
            .get_result(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e).into())
    }

    pub async fn delete(&self, conn: &mut AsyncPgConnection) -> Result<usize> {
        diesel::delete(users::table.find(self.id))
            .execute(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e).into())
    }

    pub async fn get_all(conn: &mut AsyncPgConnection) -> Result<Vec<User>> {
        users::table
            .load::<User>(conn)
            .await
            .map_err(|e| UserError::DatabaseError(e).into())
    }
    
    fn is_hashed_password(password: &str) -> bool {
        password.starts_with("$argon2")
    }
}
