import classes from "./MainHeader.module.css";

function MainHeader({ onCreateExpense }: { onCreateExpense: () => void }) {
  return (
    <header className={classes.header}>
      <h1 className={classes.logo}>Expenses</h1>
      <p>
        <button className={classes.button} onClick={onCreateExpense}>
          New Expense
        </button>
      </p>
    </header>
  );
}

export default MainHeader;
