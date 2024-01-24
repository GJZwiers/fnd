import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export interface Expense {
  id: string,
  name: string;
  amount: string;
  flexible: boolean;
}

// Get the expenses from the database on first page load, with a form to add a new expense. The state is updated on form submit.
export default function Expenses() {
  const [expenses, setExpenses] = useState<Expense[]>([]);

  useEffect(() => {
    invoke<Expense[]>("load_expenses")
      .then((expenses) => {
        setExpenses(expenses);
      })
      .catch(console.error);
  }, []);

  function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    const formData = new FormData(e.target as HTMLFormElement);
    const formProps = Object.fromEntries(formData);

    let updated_expenses: Expense[] = [
      ...expenses,
      {
        id: formProps.id as string,
        name: formProps.expenseName as string,
        amount: formProps.expenseAmount as string,
        flexible: formProps.expenseFlexible === "true",
      },
    ];

    invoke("write_new_expense", {
      expense: {
        name: formProps.expenseName,
        amount: parseFloat(formProps.expenseAmount as string),
        flex: formProps.expenseFlexible === "true",
      },
    })
      .then(() => {
        console.log("wrote new expense to database");
        setExpenses(updated_expenses);
      })
      .catch(console.error);
  }

  function handleTableClick(e: React.FocusEvent<HTMLTableCellElement>) {
    // console.log(e.target.innerHTML);
    if (e.target.id === "name") {     
      
    } else if (e.target.id === "amount") {

    } else if (e.target.id === "flexible") {

    }
    
    console.log("clicked table cell!")
  }

  return (
    <div className="grid grid-cols-2 gap-4">
      <form
        id="newExpenseForm"
        className="w-full max-w-sm"
        onSubmit={handleSubmit}
      >
        <div className="flex justify-center font-bold text-gray-700">
          <h1>New Expense</h1>
        </div>

        <div className="md:flex md:items-center mb-6">
          <div className="md:w-1/3">
            <label
              className="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4"
              htmlFor="inline-full-name"
            >
              Name
            </label>
          </div>
          <div className="md:w-2/3">
            <input
              className="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
              name="expenseName"
              id="inline-full-name"
              type="text"
              placeholder="potatoes"
            ></input>
          </div>
        </div>
        <div className="md:flex md:items-center mb-6">
          <div className="md:w-1/3">
            <label
              className="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4"
              htmlFor="inline-amount"
            >
              Amount
            </label>
          </div>
          <div className="md:w-2/3">
            <input
              className="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
              name="expenseAmount"
              id="inline-amount"
              type="text"
              placeholder="10"
            ></input>
          </div>
        </div>
        <div className="md:flex md:items-center mb-6">
          <div className="md:w-1/3">
            <label
              className="block text-gray-500 font-bold md:text-right mb-1 md:mb-0 pr-4"
              htmlFor="inline-flexible"
            >
              Flexible
            </label>
          </div>
          <div className="md:w-2/3">
            <select
              className="bg-gray-200 appearance-none border-2 border-gray-200 rounded w-full py-2 px-4 text-gray-700 leading-tight focus:outline-none focus:bg-white focus:border-purple-500"
              name="expenseFlexible"
              id="inline-flexible"
            >
              <option>true</option>
              <option>false</option>
            </select>
          </div>
        </div>

        <div className="md:flex md:items-center">
          <div className="md:w-1/3"></div>
          <div className="md:w-2/3">
            <button
              className="shadow bg-blue-500 hover:bg-purple-400 focus:shadow-outline focus:outline-none text-white font-bold py-2 px-4 rounded"
              type="submit"
              // onClick={handleClick}
            >
              Add
            </button>
          </div>
        </div>
      </form>
      <div>
        <div className="flex justify-center font-bold text-gray-700">
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
                          ID
                        </th>
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
                        <tr
                          key={i}
                          className="border-b dark:border-neutral-500"
                        >
                          <td className="whitespace-nowrap px-6 py-4 font-medium" > 
                            {v.id}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4 font-medium" onBlur={handleTableClick}>
                            {v.name}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4" suppressContentEditableWarning={true}>
                            {v.amount}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4" >
                            {v.flexible.toString()}
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
    </div>
  );
}
