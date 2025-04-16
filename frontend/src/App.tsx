import { useEffect, useState } from 'react'
import './App.css'
import Dialog, { DialogData } from './components/dialog/Dialog'
import SnipeTarget from './components/snipeTarget/SnipeTarget'
import { useSnipeStore } from './store/snipeTargetStore'
import Token from './components/token/Token'
import TokenBuyPanel from './components/tokenBuyPanel/TokenBuyPanel'

function App() {
  const [dialogs, setDialogs] = useState<DialogData[]>([])
  const targets = useSnipeStore((state) => state.targets)
  const fetchSnipeTargets = useSnipeStore((state) => state.fetchSnipeTargets)

  useEffect(() => {
    fetchSnipeTargets();
    const fetchDialogs = async () => {
      try {
        const res = await fetch('http://localhost:8001/api/v1/tg/dialogs')
        const data = await res.json()
        if (data.dialogs) {
          setDialogs(data.dialogs)
        }
      } catch (err) {
        console.error('Failed to fetch dialogs:', err)
      }
    }

    fetchDialogs()
  }, [])

  return (
    <>
      <div className='App'>
        <TokenBuyPanel onBuy={()=>{}} mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/>
        {/* <Token mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/> */}
        <div>
        {targets.map((target) => (
          <SnipeTarget key={target.targetId} {...target} />
        ))}
        </div>
      </div>
    </>
  )
}

export default App
