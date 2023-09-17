use fnd::map_items;
use fnd::read_var_expenses;
use fnd::ten_year_interests;
use fnd::total_savings;
use fnd::Sums;
use fnd::Transactions;
use std::env;
use std::format;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let toml_string = fs::read_to_string("./transactions.toml")?;
    let Transactions {
        accounts,
        expenses,
        income,
        transfers,
    } = toml::from_str(toml_string.as_str())?;

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

    println!("{:.2} total savings", total_savings);
    accounts.iter().for_each(|account| {
        println!(
            "{}",
            format!(
                "{:.2} {}: (10 yr: {:.2})",
                account.amount, account.name, account.interest
            )
        )
    });

    println!("\n{:.2} in ({:.2} flexible)", total_income, flex_income);
    println!(
        "{:.2} out ({:.2} flexible, {:.2} variable)",
        total_expenses, flex_expenses, var_expenses
    );
    println!("{:.2} moved", total_transfers);
    println!("{} free", free_income);

    Ok(())
}
