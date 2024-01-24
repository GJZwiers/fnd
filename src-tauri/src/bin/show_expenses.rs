use self::models::*;
use app::*;
use db::establish_db_connection;
use diesel::prelude::*;

fn main() {
    use self::schema::expenses::dsl::*;

    let connection = &mut establish_db_connection();
    let results = expenses
        //.filter(published.eq(true))
        .limit(50)
        .select(Expense::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for expense in results {
        println!("{}", expense.name);
        println!("{}\n", expense.amount);
    }
}
