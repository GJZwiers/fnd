use crate::schema::expenses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub flexible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = expenses)]
pub struct NewExpense {
    pub name: String,
    pub amount: f32,
    pub flexible: bool,
}
