import Image from 'next/image'
import Transactions from './read_transactions'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Transactions />
    </main>
  )
}
