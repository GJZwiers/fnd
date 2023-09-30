'use client'

import NewExpense from "./form"
import Expenses from "./list_expenses"

export default function Page() {
  function handleClick() {
    window.location.href = "/"
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <button onClick={handleClick}>Home</button>
      <div className="grid grid-cols-2 gap-4">
        <div><NewExpense /></div>
        <div><Expenses /></div>
      </div>
      
      
    </main>
  )
}
