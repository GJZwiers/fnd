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
struct Saving {
    name: String,
    amount: f32,
    interest: f32,
}

#[derive(Deserialize, Debug)]
struct Savings {
    accounts: Vec<Saving>,
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
    //fixed: f32,
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

struct SavingInfo {
    amount: f32,
    compound_interest: f32,
    name: String,
}

struct SavingsOut {
    total: f32,
    savings: Vec<SavingInfo>,
}

fn map_saving(savings: Vec<Saving>) -> SavingsOut {
    let mut total: f32 = 0.0;
    let mut vings = vec![];

    savings.iter().for_each(|s| {
        total += s.amount;
        let mut compound_interest: f32 = 0.0;
        if s.interest != 0.0 {
            compound_interest = calculate_compound_interest(s.amount, s.interest, 10)
        }
        vings.push(SavingInfo {
            amount: s.amount,
            compound_interest,
            name: s.name.clone(),
        })
    });

    SavingsOut {
        total,
        savings: vings,
    }
}

fn calculate_compound_interest(principal: f32, rate: f32, t: u32) -> f32 {
    principal * ((1. + rate).powf(t as f32)) - principal
}

fn main() {
    let toml_str = fs::read_to_string("./expenses.toml").unwrap();
    let expenses: Expenses = toml::from_str(toml_str.as_str()).unwrap();

    let Sums {
        total: mut total_expenses,
        //fixed: fixed_charges,
        flex: flex_charges,
    } = map_items(expenses.charges.fixed);
    let Sums {
        total: total_income,
        //fixed: fixed_income,
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

    let savings = map_saving(expenses.savings.accounts);

    let free_r = format!("{:.2}", total_income - total_expenses - total_transfers);
    let fixed_income_r = format!("{:.2}", total_income - flex_income);
    let fixed_charges_r = format!("{:.2}", total_expenses - flex_charges);

    println!(
        "{}",
        format!("in: {total_income} ({fixed_income_r} fixed, {flex_income} flexible)")
    );
    println!(
        "{}",
        format!("out: {total_expenses} ({fixed_charges_r} fixed {flex_charges} flexible)")
    );
    println!("{}", format!("{} moved", total_transfers));
    println!("{}", format!("{free_r} free"));
    println!("{}", format!("{} total savings\n", savings.total));

    savings.savings.iter().for_each(|saving| {
        let ten_year_interest = saving.amount + saving.compound_interest;
        let ten_r = format!("{:.2}", ten_year_interest);

        println!(
            "{}",
            format!("{}: {} (10 yr: {ten_r})", saving.name, saving.amount)
        );
    });
}
