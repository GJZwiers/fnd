import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import reactLogo from "../assets/react.svg";
import tauriLogo from "../assets/tauri.svg";
import nextLogo from "../assets/next.svg";

import { Input, Table, useAsyncList, useCollator } from "@nextui-org/react";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [payment, setName] = useState("");

  async function addEarning() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { payment }));
  }

  const [expenseMsg, setExpenseMsg] = useState("");
  const [expense, setExpense] = useState("");

  async function addExpense() {
    setExpenseMsg(await invoke("expense", { expense }));
  }

  const columns = [
    {
      key: "transaction_type",
      label: "TRANSACTION_TYPE",
      image: 
        <Image
          width={144}
          height={144}
          src={nextLogo}
          className="logo next"
          alt="Next logo"
        />
    },
    {
      key: "description",
      label: "DESCRIPTION",
      image: 
      <Image
        width={144}
        height={144}
        src={tauriLogo}
        className="logo next"
        alt="Next logo"
      />
    },
    {
      key: "amount",
      label: "AMOUNT",
      image: 
      <Image
        width={144}
        height={144}
        src={reactLogo}
        className="logo next"
        alt="Next logo"
      />
    },
  ];
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

  const [row, setRow] = useState(rows);

  const onChangeInput = (e, key) => {   
    const { name, value } = e.target

    const editData = row.map((item) =>
        item.key === key && name ? { ...item, [name]: value } : item
    )

    setRow(editData)
  }

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
      aria-label="Example static collection table"
      css={{ minWidth: "100%", height: "auto" }} //calc($space$14 * 10)
      sortDescriptor={list.sortDescriptor}
      onSortChange={list.sort}
      compact
    >
      <Table.Header>
        <Table.Column key="transaction_type" allowsSorting>
          <Image
            width={144}
            height={144}
            src={nextLogo}
            className="logo next"
            alt="Next logo"
          />
          Type
        </Table.Column>
        <Table.Column key="description" allowsSorting>
          <Image
            width={144}
            height={144}
            src={tauriLogo}
            className="logo next"
            alt="Next logo"
          />
          Description
        </Table.Column>
        <Table.Column key="amount" allowsSorting>
          <Image
            width={144}
            height={144}
            src={reactLogo}
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
                <Input
                  aria-label="test"
                  name="type"
                  value={item[columnKey]}
                  type="text"
                  onChange={(e) => onChangeInput(e, item[columnKey])}
                  />
              </Table.Cell>}
          </Table.Row>
        )}
      </Table.Body>
    </Table>

      <div className="row">
        <div>
          <input
            id="payment-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Add earning"
          />
          <button type="button" onClick={() => addEarning()}>
            Add
          </button>
        </div>
      </div>

      <p>{greetMsg}</p>

      <div className="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setExpense(e.currentTarget.value)}
            placeholder="Add expense"
          />
          <button type="button" onClick={() => addExpense()}>
            Add
          </button>
        </div>
      </div>

      <p>{expenseMsg}</p>
    </div>
  );
}

export default App;
