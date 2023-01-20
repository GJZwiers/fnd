use serde_derive::Deserialize;
use std::format;
use std::fs;

#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    amount: f32,
    adjustable: Option<bool>,
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
struct Monthly {
    months: Vec<MonthlyExpense>,
}

#[derive(Deserialize, Debug, Clone)]
struct MonthlyExpense {
    name: String,
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
            if v.adjustable.is_some() {
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

fn main() {
    let toml_str = fs::read_to_string("./expenses.toml").unwrap();
    let expenses: Expenses = toml::from_str(toml_str.as_str()).unwrap();

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

    fs::read_dir("varying").unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        let p = path.to_str().unwrap();

        if !p.ends_with(".toml") {
            let f = format!(
                "Invalid filename '{}'. Filenames must have .toml extension",
                p
            );
            panic!("{}", f)
        }

        let file = fs::read_to_string(path).unwrap();
        let monthly: Monthly = toml::from_str(file.as_str()).unwrap();

        let len = monthly.months.len();
        let sum = monthly
            .months
            .into_iter()
            .flat_map(|v| v.amounts)
            .sum::<f32>();

        let avg = sum / len as f32;
        total_expenses += avg;
    });

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
            let tyi = format!("{:.2}", account.amount + compound_interest);
            let tyi_str = format!("{}: {} (10 yr: {tyi})", account.name, account.amount);
            ten_year_interests.push(tyi_str);
        }
    });

    let free_r = format!("{:.2}", total_income - total_expenses - total_transfers);
    let fixed_income_r = format!("{:.2}", total_income - flex_income);
    let fixed_charges_r = format!("{:.2}", total_expenses - flex_charges);

    println!(
        "in: {} ({} fixed, {} flexible)",
        total_income, fixed_income_r, flex_income
    );
    println!(
        "out: {} ({} fixed {} flexible)",
        total_expenses, fixed_charges_r, flex_charges
    );
    println!("{} moved", total_transfers);
    println!("{} free", free_r);
    println!("{} total savings\n", total_savings);

    ten_year_interests.iter().for_each(|tyi| {
        println!("{}", tyi);
    });
}
