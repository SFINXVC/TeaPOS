use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::AsyncPgConnection;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;
// Import diesel macros and types
use diesel::{AsExpression, FromSqlRow};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, Output, ToSql};
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;
use diesel::result::Error as DieselError;

// Import our error types
use crate::errors::{Error, Result, DatabaseError, ValidationError};

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

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
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

// UserError has been removed in favor of the centralized error system in errors.rs

impl User {
    pub async fn create(new_user: NewUser, conn: &mut AsyncPgConnection) -> Result<User> {
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .await;
            
        if let Ok(_) = existing_user {
            return Err(Error::Validation(ValidationError::AlreadyExists(format!("Username '{}' already exists", new_user.username))));
        }
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
            .map_err(|e| Error::Database(DatabaseError::Query(e)))
    }

    pub async fn create_and_return(new_user: NewUser, conn: &mut AsyncPgConnection) -> Result<User> {
        let existing_user = users::table
            .filter(users::username.eq(&new_user.username))
            .first::<User>(conn)
            .await;

        if let Ok(_) = existing_user {
            return Err(Error::Validation(ValidationError::AlreadyExists(format!("Username '{}' already exists", new_user.username))));
        }

        let created_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
            .map_err(|e| Error::Database(DatabaseError::Query(e)))?;

        Ok(created_user)
    }

    pub async fn find_by_id(id: i32, conn: &mut AsyncPgConnection) -> Result<User> {
        users::table
            .find(id)
            .first(conn)
            .await
            .map_err(|error| {
                if let DieselError::NotFound = error {
                    Error::NotFound(format!("User with ID {} not found", id))
                } else {
                    Error::Database(DatabaseError::Query(error))
                }
            })
    }

    pub async fn find_by_username(username: &str, conn: &mut AsyncPgConnection) -> Result<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
            .await
            .map_err(|error| {
                if let DieselError::NotFound = error {
                    Error::NotFound(format!("User with username '{}' not found", username))
                } else {
                    Error::Database(DatabaseError::Query(error))
                }
            })
    }

    pub async fn update(&self, conn: &mut AsyncPgConnection) -> Result<User> {
        diesel::update(users::table.find(self.id))
            .set(self)
            .get_result(conn)
            .await
            .map_err(|e| Error::Database(DatabaseError::Query(e)))
    }

    pub async fn delete(&self, conn: &mut AsyncPgConnection) -> Result<usize> {
        diesel::delete(users::table.find(self.id))
            .execute(conn)
            .await
            .map_err(|e| Error::Database(DatabaseError::Query(e)))
    }

    pub async fn all(conn: &mut AsyncPgConnection) -> Result<Vec<User>> {
        users::table
            .load::<User>(conn)
            .await
            .map_err(|e| Error::Database(DatabaseError::Query(e)))
    }
}
