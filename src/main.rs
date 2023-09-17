use regex::Regex;
use serde_derive::Deserialize;
use std::env;
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
struct Account {
    amount: f32,
    name: String,
    interest: f32,
}

#[derive(Deserialize, Debug)]
struct Expenses {
    charges: Vec<Item>,
    income: Vec<Item>,
    accounts: Vec<Account>,
    transfers: Vec<Item>,
}

#[derive(Deserialize, Debug)]
struct VariableExpenses {
    out: Vec<MonthlyExpense>,
}

#[derive(Deserialize, Debug, Clone)]
struct MonthlyExpense {
    amounts: Vec<f32>,
    name: String,
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
            if v.flex.is_some() && v.flex.unwrap() == true {
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
    let args: Vec<String> = env::args().collect();
    let toml_str = fs::read_to_string("./transactions.toml")?;
    let expenses: Expenses = toml::from_str(toml_str.as_str())?;

    let Sums {
        total: mut total_expenses,
        flex: flex_charges,
    } = map_items(expenses.charges);
    let Sums {
        total: total_income,
        flex: flex_income,
    } = map_items(expenses.income);
    let Sums {
        total: total_transfers,
        ..
    } = map_items(expenses.transfers);

    let dir = fs::read_dir("variable");
    let mut avgs: Vec<f32> = vec![];
    match dir {
        Err(e) => {
            eprintln!("'variable' dir not found: {}", e)
        }
        Ok(d) => {
            for entry in d {
                let path = entry?.path();
                if path.is_dir() {
                    println!("Found a directory in the 'variable' folder; skipping..");
                } else if path.extension() != Some(OsStr::new("toml")) {
                    panic!(
                        "Invalid filename '{:?}'. Filenames must have .toml extension",
                        path
                    );
                }

                let var_expenses: VariableExpenses =
                    toml::from_str(fs::read_to_string(path)?.as_str())?;
                let expense_count = var_expenses.out.len() as f32;
                let expense_sum = var_expenses
                    .out
                    .into_iter()
                    .flat_map(|v| {
                        if args.len() > 2 {
                            let re = Regex::new(r"=(?<month>jan|feb|mar|apr|jun|jul|aug|sep|okt|nov|dev)").unwrap();
                            let caps = re.captures(args[2].as_str()).unwrap();

                            if caps["month"] != v.name {
                                return Vec::new()
                            }
                        }

                        v.amounts
                    })
                    .sum::<f32>();

                let avg = expense_sum / expense_count;
                avgs.push(avg);
                total_expenses += avg;
            }
        }
    }
    // fixed flex variable
    // flex = variable and able to control amount

    let total_savings = expenses
        .accounts
        .iter()
        .map(|account| account.amount)
        .sum::<f32>();

    let mut ten_year_interests: Vec<String> = vec![];
    expenses.accounts.iter().for_each(|account| {
        let compound_interest =
            calculate_compound_interest(account.amount, account.interest, 10);
        let ten_year_interest = format!(
            "{}: {} (10 yr: {:.2})",
            account.name,
            account.amount,
            account.amount + compound_interest
        );
        ten_year_interests.push(ten_year_interest);
    });

    let var_charges = avgs.iter().sum::<f32>();

    let free_income =
        format!("{:.2}", total_income - total_expenses - total_transfers);

    println!("{} in ({} flexible)", total_income, flex_income);
    println!(
        "{:.2} out ({:.2} flexible, {} variable)",
        total_expenses, flex_charges, var_charges
    );
    println!("{} moved", total_transfers);
    println!("{} free", free_income);
    println!("{} total savings\n", total_savings);

    ten_year_interests.iter().for_each(|ten_year_interest| {
        println!("{}", ten_year_interest);
    });

    Ok(())
}
