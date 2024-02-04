"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface TableDataItem {
  name: string;
  total: string;
  var: string;
  interest_rate: string;
  interest?: string;
  interest_yr?: string;
  payments: string;
}

interface TableData {
  accounts: TableDataItem[];
  transactions: TableDataItem[];
}

export default function Transactions() {
  const [transactions, setTransactions] = useState<TableDataItem[]>([]);
  const [accounts, setAccounts] = useState<TableDataItem[]>([]);

  useEffect(() => {
    invoke<TableData>("read_transactions")
      .then((tableData) => {
        setTransactions(tableData.transactions);
        setAccounts(tableData.accounts);
      })
      .catch(console.error);
  }, []);

  return (
    <div>
      <div className="flex justify-center">
        <h1>Transaction Summary</h1>
      </div>

      <div className="item-container">
        <div className="flex flex-col">
          <div className="overflow-x-auto sm:-mx-6 lg:-mx-8">
            <div className="inline-block min-w-full py-2 sm:px-6 lg:px-8">
              <div className="overflow-hidden">
                <table className="min-w-full text-left text-sm font-light">
                  <thead className="border-b font-light dark:border-neutral-500">
                    <tr>
                      <th scope="col" className="px-6 py-4">
                        Type
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Total
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Variable
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {transactions.map((v, i, a) => (
                      <tr key={i} className="border-b dark:border-neutral-500 ">
                        <td className="whitespace-nowrap px-6 py-4 font-light">
                          {v.name}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.total}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">{v.var}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>

                <br></br>
                <div className="flex justify-center">
                  <h1>Account Summary</h1>
                </div>

                <table className="min-w-full text-left text-sm font-light">
                  <thead className="border-b font-light dark:border-neutral-500">
                    <tr>
                      <th scope="col" className="px-6 py-4">
                        Account
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Principal Amount
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Interest Rate
                      </th>
                      <th scope="col" className="px-6 py-4">
                        Amount After Interest
                      </th>
                      <th scope="col" className="px-6 py-4">
                        After Interest and Deposits
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {accounts.map((v, i, a) => (
                      <tr key={i} className="border-b dark:border-neutral-500">
                        <td className="whitespace-nowrap px-6 py-4 font-light">
                          {v.name}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.total}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.interest_rate}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.interest}{" "}
                          {v.interest_yr
                            ? "(" + v.interest_yr + "y)"
                            : undefined}
                        </td>
                        <td className="whitespace-nowrap px-6 py-4">
                          {v.payments}
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
