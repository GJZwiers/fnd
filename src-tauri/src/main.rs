// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

use app::map_items;
use app::models::Expense;
use app::models::NewExpense;
use app::read_var_expenses;
use app::ten_year_interests;
use app::total_savings;
use app::ExpenseResponse;
use app::Item;
use app::Sums;
use app::TableData;
use app::TableDataItem;
use app::Transactions;
use std::env;
use std::format;
use std::fs;

use diesel::prelude::*;

use db::establish_db_connection;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            db::init();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_transactions,
            get_expenses_from_file,
            write_new_expense,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn load_expenses() {
//     use app::schema::expenses::dsl::*;

//     let conn = &mut establish_db_connection();

//     let results = expenses
//         //.filter(flexible.eq(false))
//         .limit(25)
//         .select(Expense::as_select())
//         .load(conn)
//         .expect("Error loading expenses");

//     println!("Displaying {} expenses", results.len());
//     for expense in results {
//         println!("{}", expense.name);
//         println!("-----------\n");
//         println!("amount {}", expense.amount);
//         println!("flex {}\n", expense.flexible);
//     }
// }

#[tauri::command]
fn write_new_expense(expense: Item) {
    use app::schema::expenses;

    let flex = expense.flex.is_some();

    let new_expense = NewExpense {
        name: expense.name,
        amount: expense.amount,
        flexible: flex,
    };

    let conn = &mut establish_db_connection();

    diesel::insert_into(expenses::table)
        .values(&new_expense)
        .returning(Expense::as_returning())
        .get_result(conn)
        .expect("Error saving new expense");
}

// #[tauri::command]
// fn update_expense(id: i32) {
//     use app::schema::expenses::dsl::{expenses, flexible};

//     let conn = &mut establish_db_connection();

//     let expense = diesel::update(expenses.find(id))
//         .set(flexible.eq(true))
//         .returning(Expense::as_returning())
//         .get_result(conn)
//         .unwrap();
//     println!("Updated expense {}", expense.name);
// }

// #[tauri::command]
// fn delete_expense(pattern: String) {
//     use app::schema::expenses::dsl::*;

//     let conn = &mut establish_db_connection();

//     let num_deleted = diesel::delete(expenses.filter(name.like(pattern)))
//         .execute(conn)
//         .expect("Error deleting posts");

//     println!("Deleted {} expenses", num_deleted);
// }

#[tauri::command]
fn get_expenses_from_file() -> Vec<ExpenseResponse> {
    let toml_string = fs::read_to_string("src/transactions.toml").unwrap();
    let Transactions { expenses, .. } =
        toml::from_str(toml_string.as_str()).unwrap();

    let response = expenses
        .iter()
        .map(|expense| {
            let flex = if expense.flex.is_none() {
                "false".to_string()
            } else {
                "true".to_string()
            };
            ExpenseResponse {
                name: expense.name.clone(),
                amount: format!("{:.2}", expense.amount),
                flex,
            }
        })
        .collect::<Vec<ExpenseResponse>>();

    response
}

#[tauri::command]
fn read_transactions() -> TableData {
    let args: Vec<String> = env::args().collect();
    let toml_string = fs::read_to_string("src/transactions.toml").unwrap();
    let Transactions {
        accounts,
        expenses,
        income,
        transfers,
    } = toml::from_str(toml_string.as_str()).unwrap();

    let Sums {
        total: mut total_expenses,
        flex: flex_expenses,
    } = map_items(expenses);

    let var_expenses = read_var_expenses(args).unwrap().iter().sum::<f32>();
    total_expenses += var_expenses;

    let Sums {
        total: total_income,
        flex: flex_income,
    } = map_items(income);
    let Sums {
        total: total_transfers,
        ..
    } = map_items(transfers);

    let free_income =
        format!("{:.2}", total_income - total_expenses - total_transfers)
            .parse::<f32>()
            .unwrap();

    let accounts = ten_year_interests(&accounts);
    let total_savings = total_savings(&accounts);

    let rounded_income = format!("{:.2}", total_income);
    let rounded_expenses = format! {"{:.2}", total_expenses};
    let rounded_flex_income = format!("{:.2}", flex_income);
    let rounded_flex_expense = format!("{:.2}", flex_expenses);
    let rounded_var_expenses = format!("{:.2}", var_expenses);
    let rounded_transfers = format!("{:.2}", total_transfers);
    let rounded_free_income = format!("{:.2}", free_income);

    let table_rows = vec![
        TableDataItem {
            name: "income".to_string(),
            total: rounded_income,
            flex: rounded_flex_income,
            var: "0.00".to_string(),
            interest: None,
        },
        TableDataItem {
            name: "expenses".to_string(),
            total: rounded_expenses,
            flex: rounded_flex_expense,
            var: rounded_var_expenses,
            interest: None,
        },
        TableDataItem {
            name: "transfers".to_string(),
            total: rounded_transfers,
            flex: "0.00".to_string(),
            var: "0.00".to_string(),
            interest: None,
        },
        TableDataItem {
            name: "free".to_string(),
            total: rounded_free_income,
            flex: "0.00".to_string(),
            var: "0.00".to_string(),
            interest: None,
        },
    ];

    let mut accs = vec![];
    accounts.iter().for_each(|account| {
        accs.push(TableDataItem {
            name: account.name.clone(),
            total: format!("{:.2}", account.amount),
            interest: Some(format!("{:.2}", account.interest)),
            flex: "".to_string(),
            var: "".to_string(),
        })
    });

    accs.push(TableDataItem {
        name: "total savings".to_string(),
        total: format!("{:.2}", total_savings),
        flex: "-".to_string(),
        var: "-".to_string(),
        interest: None,
    });

    TableData {
        accounts: accs,
        transactions: table_rows,
    }
}
