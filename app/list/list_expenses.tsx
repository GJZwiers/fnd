import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Expense {
  name: string;
  amount: string;
  flex: boolean;
}

export default function Expenses() {
  const [expenses, setExpenses] = useState<Expense[]>([]);

  useEffect(() => {
    invoke<Expense[]>("get_expenses_from_file")
      .then((expenses) => {
        setExpenses(expenses);
      })
      .catch(console.error);
  }, []);

  return (
    <div>
      <div className="flex justify-center">
        <h1>Expenses</h1>
      </div>

      <div className="item-container">
        <div className="flex flex-col">
          <div className="overflow-x-auto sm:-mx-6 lg:-mx-8">
            <div className="inline-block min-w-full py-2 sm:px-6 lg:px-8">
              <div className="overflow-hidden">
                <table className="min-w-full text-left text-sm font-light">
                  <thead className="border-b font-medium dark:border-neutral-500">
                    <tr>
                      <th scope="col" className="px-6 py-4">
                        Name
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Amount
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Flexible
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {expenses.map((v, i, a) => (
                      <tr key={i} className="border-b dark:border-neutral-500">
                        <td className="whitespace-nowrap px-6 py-4 font-medium">
                          {v.name}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.amount}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.flex}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
