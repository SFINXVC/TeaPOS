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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "superadmin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            "employee" => Ok(UserRole::Employee),
            _ => Err(format!("Unknown role: {}", s)),
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
    pub password: String,
    pub whatsapp: String,
    pub role: UserRole,
}

impl User {
    pub async fn create(new_user: NewUser, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .await
    }

    pub async fn find_by_id(id: i32, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        users::table.find(id).first(conn).await
    }

    pub async fn find_by_username(username: &str, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
            .await
    }

    pub async fn update(&self, conn: &mut AsyncPgConnection) -> QueryResult<User> {
        diesel::update(users::table.find(self.id))
            .set(self)
            .get_result(conn)
            .await
    }

    pub async fn delete(&self, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
        diesel::delete(users::table.find(self.id)).execute(conn).await
    }

    pub async fn all(conn: &mut AsyncPgConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn).await
    }
}
