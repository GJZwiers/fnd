import classes from './Expense.module.css';

function Expense({ amount, name}: { amount: string, name: string}) {
  return <li className={classes.post}>
    <p className={classes.amount}>{amount}</p>
    <p className={classes.text}>{name}</p>
  </li>
}

export default Expense;
