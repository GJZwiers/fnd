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

  function nameChangeHandler(event: React.ChangeEvent<HTMLTextAreaElement>) {
    setExpenseName(event.target.value);
  }

  function amountChangeHandler(event: React.ChangeEvent<HTMLInputElement>) {
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
        <label htmlFor="name">Expense Name</label>
        <textarea
          id="name"
          required
          rows={3}
          onChange={nameChangeHandler}
        />
      </p>
      <p>
        <label htmlFor="amount">Amount</label>
        <input type="text" id="amount" required onChange={amountChangeHandler} />
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
