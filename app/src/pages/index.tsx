import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import scale from "../assets/scale.svg";
import quill from "../assets/quill.svg";
import gold from "../assets/gold.svg";

import { Table, useAsyncList, useCollator } from "@nextui-org/react";

function App() {
  const [rowName, setRowName] = useState("");
  const [row, setRow] = useState("");

  async function addRow() {
    rows.push({
      key: "0",
      transaction_type: "in",
      description: "foo",
      amount: "0",
    })
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setRowName(await invoke("add_row", { row }));
  }

  const rows = [
    {
      key: "1",
      transaction_type: "out",
      description: "rent",
      amount: "100",
    },
    {
      key: "2",
      transaction_type: "out",
      description: "internet",
      amount: "50",
    },
    {
      key: "3",
      transaction_type: "out",
      description: "food",
      amount: "20",
    },
    {
      key: "4",
      transaction_type: "in",
      description: "wages",
      amount: "200",
    },
  ];

  const collator = useCollator({ numeric: true });

  async function load({ signal }) {
    return {
      items: rows,
    };
  }

  async function sort({ items, sortDescriptor }) {
    return {
      items: items.sort((a, b) => {
        let first = a[sortDescriptor.column];
        let second = b[sortDescriptor.column];
        let cmp = collator.compare(first, second);
        if (sortDescriptor.direction === "descending") {
          cmp *= -1;
        }
        return cmp;
      }),
    };
  }

  const list = useAsyncList({ load, sort });
  return (
    <div className="container">
      <h2>Treasure Cove</h2>

    <Table
      aria-label="Le Table"
      css={{ minWidth: "100%", height: "auto" }} // calc($space$14 * 10)
      sortDescriptor={list.sortDescriptor}
      onSortChange={list.sort}
      compact
    >
      <Table.Header>
        <Table.Column key="transaction_type" allowsSorting>
          <Image
            width={144}
            height={144}
            src={gold}
            className="logo next"
            alt="Next logo"
          />
          Type
        </Table.Column>
        <Table.Column key="description" allowsSorting>
          <Image
            width={144}
            height={144}
            src={quill}
            className="logo next"
            alt="Next logo"
          />
          Description
        </Table.Column>
        <Table.Column key="amount" allowsSorting>
          <Image
            width={144}
            height={144}
            src={scale}
            className="logo next"
            alt="Next logo"
          />
          Amount
        </Table.Column>

      </Table.Header>
      <Table.Body items={list.items} loadingState={list.loadingState}>
        {(item) => (
          <Table.Row key={item.description}>
            {(columnKey) => 
              <Table.Cell>
                {item[columnKey]}
              </Table.Cell>}
          </Table.Row>
        )}
      </Table.Body>
    </Table>

      <div className="row">
        <div>
          <input
            id="row-input"
            onChange={(e) => setRow(e.currentTarget.value)}
            placeholder="new"
          />
          <button type="button" onClick={() => addRow()}>
            Add
          </button>
        </div>
      </div>

      <p>{rowName}</p>
    </div>
  );
}

export default App;
