use crate::schema::expenses;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub flexible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Insertable)]
#[diesel(table_name = expenses)]
pub struct NewExpense {
    pub name: String,
    pub amount: f32,
    pub flexible: bool,
}
