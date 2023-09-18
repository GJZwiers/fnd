import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import scale from "../assets/scale.svg";
import quill from "../assets/quill.svg";
import gold from "../assets/gold.svg";

import { Table, useAsyncList, useCollator } from "@nextui-org/react";

function App() {
  return (
    <div className="container">
      <h2>Treasure Cove</h2>

      <div className="row">
        <div>
          <span>One</span>
          <br></br>
          <span>Two</span>
        </div>
      </div>
    </div>
  );
}

export default App;
