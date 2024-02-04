import NewExpense from "./NewExpense";
import Expense from "./Expense";
import Modal from "./Modal";
import { useState, useEffect } from "react";
import classes from "./ExpenseList.module.css";

type Expense = {
  body: string,
  name: string,
}

function ExpenseList({
  isPosting,
  onStopPosting,
}: {
  isPosting: boolean;
  onStopPosting: () => void;
}) {
  const [posts, setPosts] = useState<Expense[]>([]);

  // useEffect(() => {
  //   async function fetchPosts() {
  //     const response = await fetch('http://localhost:8080/posts');
  //     const data = await response.json();
  //     setPosts(data.expenses);
  //   }

  //   fetchPosts();
  // }, []);

  function addExpenseHandler(postData: { name: string; body: string }) {
    console.log(postData);
    // fetch('http://localhost:8080/posts', {
    //   method: 'POST',
    //   body: JSON.stringify(postData),
    //   headers: {
    //     'Content-Type': 'application/json'
    //   }
    // })
    setPosts((existingPosts) => [postData, ...existingPosts]);
  }

  return (
    <>
      {isPosting && (
        <Modal onClose={onStopPosting}>
          <NewExpense
            onCancel={onStopPosting}
            onAddExpense={addExpenseHandler}
          />
        </Modal>
      )}
      {posts.length > 0 && (
        <ul className={classes.expenses}>
          {posts.map((post) => (
            <Expense key={post.body} name={post.name} amount={post.body} />
          ))}
        </ul>
      )}
      {posts.length === 0 && (
        <div style={{ textAlign: "center", color: "white" }}>
          <h2>There are no expenses yet.</h2>
          <p>Start adding some!</p>
        </div>
      )}
    </>
  );
}

export default ExpenseList;
