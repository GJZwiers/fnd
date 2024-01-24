use std::{ffi::OsStr, fs, path::PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

pub mod db;
pub mod models;
pub mod schema;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDataItem {
    pub name: String,
    pub total: String,
    pub flex: String,
    pub var: String,
    pub interest: Option<String>,
    pub interest_yr: Option<String>,
    pub payments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableData {
    pub accounts: Vec<TableDataItem>,
    pub transactions: Vec<TableDataItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub amount: f32,
    pub flex: Option<bool>,
}

#[derive(Serialize)]
pub struct ExpenseResponse {
    pub name: String,
    pub amount: String,
    pub flex: String,
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
            if item.flex.is_some() && item.flex.unwrap() {
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
    println!("{:?}", args);
    let expense_count = var_expenses.out.len() as f32;
    let expense_sum = var_expenses
        .out
        .into_iter()
        .flat_map(|v| {
            if args.len() > 1 {
                let re = Regex::new(
                    r"=(?<month>jan|feb|mar|apr|jun|jul|aug|sep|okt|nov|dev)",
                )
                .unwrap();
                let caps = re.captures(args[1].as_str()).unwrap();

                if caps["month"] != v.name {
                    return Vec::new();
                }
            }

            v.amounts
        })
        .sum::<f32>();

    Ok(expense_sum / expense_count)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub amount: f32,
    pub name: String,
    pub interest: f32,
    pub interest_yr: u32,
    pub deposit: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountResult {
    pub amount: f32,
    pub name: String,
    pub interest: f32,
    pub payments: f32,
    pub interest_yr: u32,
    pub deposit: f32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transactions {
    pub expenses: Vec<Item>,
    pub income: Vec<Item>,
    pub accounts: Vec<Account>,
    pub transfers: Vec<Item>,
}

pub fn total_savings(accounts: &[AccountResult]) -> f32 {
    accounts.iter().map(|account| account.amount).sum::<f32>()
}

pub fn ten_year_interests(accounts: &Vec<Account>) -> Vec<AccountResult> {
    let mut ten_year_interests: Vec<AccountResult> =
        Vec::with_capacity(accounts.len());

    accounts.iter().for_each(|account| {
        let compound_interest =
            calculate_compound_deposits(account.amount, account.deposit, account.interest, account.interest_yr);
            //calculate_compound_interest(account.amount, account.interest, account.interest_yr);

        let ten_year_interest = AccountResult {
            name: account.name.clone(),
            amount: account.amount,
            interest: account.amount + compound_interest.f1,
            payments: account.amount + compound_interest.f1 + compound_interest.f2,
            interest_yr: account.interest_yr,
            deposit: account.deposit,
        };
        ten_year_interests.push(ten_year_interest);
    });
    ten_year_interests
}

pub fn calculate_compound_interest(principal: f32, rate: f32, t: u32) -> f32 {
    principal * ((1. + rate).powf(t as f32)) - principal
}

pub struct InterestObject {
    pub f1: f32,
    pub f2: f32,
}

// A = PMT * (((1 + r)t - 1) / r) * (1 + r)
// A = PMT * (((1 + r/n)n*t -1) / (r/n)) * (1 + r/n)
pub fn calculate_compound_deposits(principal: f32, pmt: f32, rate: f32, t: u32) -> InterestObject {
    let n = 12 as f32;
    let f1 = principal * ((1. + rate).powf(t as f32)) - principal;
    let f2 = pmt * (((1. + rate / n).powf(n * t as f32) - 1.) / (rate/n)) * (1. + (rate/n));
    InterestObject {
        f1,
        f2,
    }
    // f1 + f2
}

// A = P * (1 + r/n)n*t
// A = P * (1 + r)t with n = 1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_items_sums_amounts() {
        let sums = map_items(vec![
            Item {
                name: "foo".to_string(),
                amount: 1.0,
                flex: None,
            },
            Item {
                name: "foo".to_string(),
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
                name: "foo".to_string(),
                amount: 1.0,
                flex: Some(true),
            },
            Item {
                name: "foo".to_string(),
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
                name: "foo".to_string(),
                amount: 1.0,
                flex: Some(false),
            },
            Item {
                name: "foo".to_string(),
                amount: 1.0,
                flex: None,
            },
        ]);
        assert_eq!(sums.total, 2.0);
        assert_eq!(sums.flex, 0.0);
    }
}
