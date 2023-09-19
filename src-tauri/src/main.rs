// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::map_items;
use app::read_var_expenses;
use app::ten_year_interests;
use app::total_savings;
use app::Sums;
use app::Transactions;
use std::env;
use std::format;
use std::fs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_transactions])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn read_transactions() -> Vec<String> {
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
        format!("{:.2}", total_income - total_expenses - total_transfers);

    let accounts = ten_year_interests(&accounts);
    let total_savings = total_savings(&accounts);

    let mut data = vec![
        format!("{:.2} total savings", total_savings),
        format!("\n{:.2} in ({:.2} flexible)", total_income, flex_income),
        format!(
            "{:.2} out ({:.2} flexible, {:.2} variable)",
            total_expenses, flex_expenses, var_expenses
        ),
        format!("{:.2} moved", total_transfers),
        format!("{} free", free_income),
    ];

    accounts.iter().for_each(|account| {
        data.push(format!(
            "{:.2} {}: (10 yr: {:.2})",
            account.amount, account.name, account.interest
        ))
    });
    data
}
