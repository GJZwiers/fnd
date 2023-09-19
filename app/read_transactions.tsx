'use client'

import { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'

export default function Transactions() {
    const [transactions, setTransactions] = useState<string[]>([]);

    useEffect(() => {
        invoke<string[]>('read_transactions')
            .then((data) => {
                setTransactions(data)
            })
            .catch(console.error)
    }, [])

  return (
    <div>
      <h1>Treasure Cove</h1>
      <div className='item-container'>
        {transactions.map((v, i, a) => {
            return <div key={i}>{v}</div>
        })}
      </div>
    </div>
  );
};
 