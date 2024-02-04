"use client";

import { useState } from "react";
import ExpenseList from "./ExpenseList";
import MainHeader from "./MainHeader";
import Link from "next/link";

export default function Page() {
  const [modalIsVisible, setModalIsVisible] = useState(false);

  function showModalHandler() {
    setModalIsVisible(true);
  }

  function hideModalHandler() {
    setModalIsVisible(false);
  }

  return (
    <main>
      <Link href="/" className="text-blue-600">
        Overview
      </Link>
      <MainHeader onCreateExpense={showModalHandler} />
      <ExpenseList
        isPosting={modalIsVisible}
        onStopPosting={hideModalHandler}
      />
    </main>
  );
}
