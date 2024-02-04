# fnd

This is a just-for-fun personal finance app, built with Next.js and Tauri.
Right now the app has two pages, an overview page that shows income/expenses/accounts and a page to view and add expenses. The app is very much work-in-progress, I use it as a way to learn more about JavaScript frontend development and backend development with Rust.

Right now the app requires a `transactions.toml` file that lists fixed monthly wages/expenses etc. See `example.transactions.toml` for an example. In the future the app will move to using a SQLite database.

If there are one-time expenses or expenses that vary from month to month, like groceries, you can create a folder `variable` and add `.toml` files with any name. See `example.variable.toml` for an example. The app will calculate the average monthly amount and apply it in the expenses overview.

## Setup

To run the project for development you have to [install Rust](https://www.rust-lang.org/tools/install) and then [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/next-js#create-the-rust-project). Clone the project from GitHub to your local machine and use `cargo tauri dev` to start the app.

## Options

If the program is run with the `--month={month}` argument only the one-time expenses for that month will be used, for example `--month=jan`.
