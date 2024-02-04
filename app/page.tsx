"use client";

import Transactions from "./read_transactions";
import Link from "next/link";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24 bg-white">
      <Link href="/expenses" className="text-blue-600">
        Expense List
      </Link>
      <Transactions />
    </main>
  );
}
