use serde_derive::Deserialize;
use std::ffi::OsStr;
use std::format;
use std::fs;
use std::io::Result;

#[derive(Deserialize, Debug)]
struct Item {
    amount: f32,
    flex: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Transactions {
    fixed: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Transfers {
    transfers: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct Expenses {
    charges: Transactions,
    income: Transactions,
    savings: Savings,
    transfers: Transfers,
}

#[derive(Deserialize, Debug)]
struct Savings {
    accounts: Vec<Account>,
}

#[derive(Deserialize, Debug)]
struct Account {
    name: String,
    amount: f32,
    interest: f32,
}

#[derive(Deserialize, Debug)]
struct VariableExpenses {
    expenses: Vec<MonthlyExpense>,
}

#[derive(Deserialize, Debug, Clone)]
struct MonthlyExpense {
    amounts: Vec<f32>,
}

struct Sums {
    total: f32,
    flex: f32,
}

fn map_items(items: Vec<Item>) -> Sums {
    let mut flex = 0.0;
    let sum = items
        .iter()
        .map(|v| {
            if v.flex.is_some() {
                flex += v.amount
            }
            v.amount
        })
        .sum::<f32>();

    Sums { total: sum, flex }
}

fn calculate_compound_interest(principal: f32, rate: f32, t: u32) -> f32 {
    principal * ((1. + rate).powf(t as f32)) - principal
}

fn main() -> Result<()> {
    let toml_str = fs::read_to_string("./expenses.toml")?;
    let expenses: Expenses = toml::from_str(toml_str.as_str())?;

    let Sums {
        total: mut total_expenses,
        flex: flex_charges,
    } = map_items(expenses.charges.fixed);
    let Sums {
        total: total_income,
        flex: flex_income,
    } = map_items(expenses.income.fixed);
    let Sums {
        total: total_transfers,
        ..
    } = map_items(expenses.transfers.transfers);

    for entry in fs::read_dir("variable")? {
        let path = entry?.path();
        if path.is_dir() {
            println!("Found a directory in the 'varying' folder; skipping..");
        } else if path.extension() != Some(OsStr::new("toml")) {
            panic!(
                "Invalid filename '{:?}'. Filenames must have .toml extension",
                path
            );
        }

        let file = fs::read_to_string(path)?;
        let var_expenses: VariableExpenses = toml::from_str(file.as_str())?;
        let expense_count = var_expenses.expenses.len() as f32;
        let expense_sum = var_expenses
            .expenses
            .into_iter()
            .flat_map(|v| v.amounts)
            .sum::<f32>();

        let avg = expense_sum / expense_count;
        total_expenses += avg;
    }

    let total_savings = expenses
        .savings
        .accounts
        .iter()
        .map(|account| account.amount)
        .sum::<f32>();

    let mut ten_year_interests: Vec<String> = vec![];
    expenses.savings.accounts.iter().for_each(|account| {
        if account.interest != 0.0 {
            let compound_interest =
                calculate_compound_interest(account.amount, account.interest, 10);
            let tyi = format!(
                "{}: {} (10 yr: {:.2})",
                account.name,
                account.amount,
                account.amount + compound_interest
            );
            ten_year_interests.push(tyi);
        }
    });

    let free_income = format!("{:.2}", total_income - total_expenses - total_transfers);
    let fixed_income = format!("{:.2}", total_income - flex_income);
    let fixed_charges = format!("{:.2}", total_expenses - flex_charges);

    println!(
        "in: {} ({} fixed, {} flexible)",
        total_income, fixed_income, flex_income
    );
    println!(
        "out: {} ({} fixed {} flexible)",
        format!("{:.2}", total_expenses), fixed_charges, flex_charges
    );
    println!("{} moved", total_transfers);
    println!("{} free", free_income);
    println!("{} total savings\n", total_savings);

    ten_year_interests.iter().for_each(|tyi| {
        println!("{}", tyi);
    });

    Ok(())
}
