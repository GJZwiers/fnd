use std::{ffi::OsStr, fs, path::PathBuf};

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub amount: f32,
    pub flex: Option<bool>,
}

pub struct Sums {
    pub total: f32,
    pub flex: f32,
}

pub fn map_items(items: Vec<Item>) -> Sums {
    let mut flex = 0.;
    let sum = items
        .iter()
        .map(|item| {
            if item.flex.is_some() && item.flex.unwrap() == true {
                flex += item.amount
            }
            item.amount
        })
        .sum::<f32>();

    Sums { total: sum, flex }
}

#[derive(Deserialize, Debug)]
struct MonthlyExpense {
    amounts: Vec<f32>,
    name: String,
}

pub fn read_var_expenses(
    args: Vec<String>,
) -> Result<Vec<f32>, std::io::Error> {
    let mut avgs: Vec<f32> = vec![];
    match fs::read_dir("src/variable") {
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

                avgs.push(avg_var_expense(&args, path).unwrap());
            }
        }
    }
    Ok(avgs)
}

#[derive(Deserialize, Debug)]
struct VariableExpenses {
    out: Vec<MonthlyExpense>,
}

pub fn avg_var_expense(
    args: &Vec<String>,
    path: PathBuf,
) -> Result<f32, std::io::Error> {
    let var_expenses: VariableExpenses = toml::from_str(
        match fs::read_to_string(path) {
            Ok(it) => it,
            Err(err) => return Err(err),
        }
        .as_str(),
    )?;

    let expense_count = var_expenses.out.len() as f32;
    let expense_sum = var_expenses
        .out
        .into_iter()
        .flat_map(|v| {
            if args.len() > 2 {
                let re = Regex::new(
                    r"=(?<month>jan|feb|mar|apr|jun|jul|aug|sep|okt|nov|dev)",
                )
                .unwrap();
                let caps = re.captures(args[2].as_str()).unwrap();

                if caps["month"] != v.name {
                    return Vec::new();
                }
            }

            v.amounts
        })
        .sum::<f32>();

    Ok(expense_sum / expense_count)
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub amount: f32,
    pub name: String,
    // either the interest rate or the amount of generated interest
    pub interest: f32,
}

#[derive(Deserialize, Debug)]
pub struct Transactions {
    pub expenses: Vec<Item>,
    pub income: Vec<Item>,
    pub accounts: Vec<Account>,
    pub transfers: Vec<Item>,
}

pub fn total_savings(accounts: &Vec<Account>) -> f32 {
    accounts.iter().map(|account| account.amount).sum::<f32>()
}

pub fn ten_year_interests(accounts: &Vec<Account>) -> Vec<Account> {
    let mut ten_year_interests: Vec<Account> =
        Vec::with_capacity(accounts.len());

    accounts.iter().for_each(|account| {
        let compound_interest =
            calculate_compound_interest(account.amount, account.interest, 10);

        let ten_year_interest = Account {
            name: account.name.clone(),
            amount: account.amount,
            interest: account.amount + compound_interest,
        };
        ten_year_interests.push(ten_year_interest);
    });
    ten_year_interests
}

pub fn calculate_compound_interest(principal: f32, rate: f32, t: u32) -> f32 {
    principal * ((1. + rate).powf(t as f32)) - principal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_items_sums_amounts() {
        let sums = map_items(vec![
            Item {
                amount: 1.0,
                flex: None,
            },
            Item {
                amount: 1.0,
                flex: None,
            },
        ]);
        assert_eq!(sums.total, 2.0);
        assert_eq!(sums.flex, 0.0);
    }

    #[test]
    fn test_map_items_flex_true() {
        let sums = map_items(vec![
            Item {
                amount: 1.0,
                flex: Some(true),
            },
            Item {
                amount: 1.0,
                flex: None,
            },
        ]);
        assert_eq!(sums.total, 2.0);
        assert_eq!(sums.flex, 1.0);
    }

    #[test]
    fn test_map_items_flex_false() {
        let sums = map_items(vec![
            Item {
                amount: 1.0,
                flex: Some(false),
            },
            Item {
                amount: 1.0,
                flex: None,
            },
        ]);
        assert_eq!(sums.total, 2.0);
        assert_eq!(sums.flex, 0.0);
    }
}
