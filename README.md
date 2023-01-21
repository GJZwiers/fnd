# fnd

A tool to get an overview of your personal finances by writing down expenses in a `.toml` file.

You can write down fixed monthly expenses in `transactions.toml`:

```toml
# Amounts that are received.
income = [
    { name = "wages", amount = 1234.56 }
]

# Amounts that need to be payed.
charges = [
    { name = "rent", amount = 123 },
    { name = "energy", amount = 456 },
]

# Amounts that are moved to other accounts (savings, investment, etc.).
transfers = [
    { name = "savings account", amount = 123 }
]

# Amounts that are in accounts.
accounts = [
    { name = "my savings account", amount = 4567, interest = 0.02 },
]
```

If there are expenses that vary from month to month, you can create a folder `variable` and add `.toml` files with any name. if an expense occurs a few times a month, e.g. groceries, you can add the indivual amounts to a list. The program will calculate an average amount over all the months and apply this in the expenses overview.

`groceries.toml`:

```toml
expenses = [
    { name = "dec", amounts = [
        20,
        30,
        25,
        # ..
    ]},
    { name = "nov", amounts = [
        22,
        45,
        26,
        # ..
    ]}
]
```
