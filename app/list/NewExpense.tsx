import { useState } from "react";
import classes from "./NewExpense.module.css";

function NewExpense({
  onCancel,
  onAddExpense,
}: {
  onCancel: () => void;
  onAddExpense: (postData: { body: string; name: string }) => void;
}) {
  const [expenseName, setExpenseName] = useState("");
  const [expenseAmount, setExpenseAmount] = useState("");

  function nameChangeHandler(event: React.ChangeEvent<HTMLInputElement>) {
    setExpenseName(event.target.value);
  }

  function amountChangeHandler(event: React.ChangeEvent<HTMLTextAreaElement>) {
    setExpenseAmount(event.target.value);
  }

  function submitHandler(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const postData = {
      body: expenseAmount,
      name: expenseName,
    };
    onAddExpense(postData);
    onCancel();
  }

  return (
    <form className={classes.form} onSubmit={submitHandler}>
      <p>
        <label htmlFor="amount">Amount</label>
        <textarea
          id="amount"
          required
          rows={3}
          onChange={amountChangeHandler}
        />
      </p>
      <p>
        <label htmlFor="name">Expense name</label>
        <input type="text" id="name" required onChange={nameChangeHandler} />
      </p>
      <p className={classes.actions}>
        <button type="button" onClick={onCancel}>
          Cancel
        </button>
        <button>Submit</button>
      </p>
    </form>
  );
}

export default NewExpense;
