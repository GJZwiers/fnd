'use client'

import Transactions from './read_transactions'

export default function Home() {
  function handleClick() {
    window.location.href = "/list"
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24 bg-white">
      <button className="text-blue-600" onClick={handleClick}>Expense List</button>
      <Transactions />
    </main>
  )
}
