"use client";

import { useState } from "react";
import ExpenseList from "./ExpenseList";
import Expenses from "./Expenses";
import MainHeader from "./MainHeader";

export default function Page() {
  const [modalIsVisible, setModalIsVisible] = useState(false);

  function showModalHandler() {
    setModalIsVisible(true);
  }

  function hideModalHandler() {
    setModalIsVisible(false);
  }

  function handleClick() {
    window.location.href = "/";
  }

  return (
    <main>
      <MainHeader onCreateExpense={showModalHandler} />
      <ExpenseList
        isPosting={modalIsVisible}
        onStopPosting={hideModalHandler}
      />
    </main>
  );
}

{
  /* <button className="text-blue-600" onClick={handleClick}>Home</button> */
}
