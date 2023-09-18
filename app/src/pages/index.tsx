import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [msg, setMsg] = useState("");
  const [exp, setExpense] = useState("");

  async function expense() {
    setMsg(await invoke("expense", { exp }));
  }

  return (
    <div className="container">
      <h2>Treasure Cove</h2>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          expense();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setExpense(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{msg}</p>
    </div>
  );
}

export default App;
