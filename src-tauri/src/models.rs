use chrono::{NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::todo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub date: NaiveDate
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todo)]
pub struct NewTodo {
    pub text: String,
    pub date: NaiveDate
}

impl NewTodo {
    pub fn new(text: String, date: NaiveDate) -> NewTodo {
        Self{ text, date }
    }
}