use diesel::prelude::*;
use serde::Deserialize;
use chrono::prelude::*;

#[derive(Queryable, Selectable, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn to_json_string(&self) -> String {
        format!(
            "{{\"id\": {}, \"username\": \"{}\", \"password_hash\": \"{}\"}}",
            self.id, self.username, self.password_hash
        )
    }
}

#[derive(Queryable, Selectable, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::token_key)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenKey {
    pub id: i32,
    pub user_id: i32,
    pub token: Vec<u8>,
    pub expiration: NaiveDateTime,
}