'use client'

import Editable from "./Editable"
import Expenses from "./expenses"

export default function Page() {
  function handleClick() {
    window.location.href = "/"
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <button onClick={handleClick}>Home</button>
      <div className="App">
      </div>
        <div><Editable /></div>
    </main>
  )
}
