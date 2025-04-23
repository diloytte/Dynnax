import { useEffect, useState } from 'react'
import './App.css'
import { DialogData } from './components/dialog/Dialog'
import SnipeTarget from './components/snipeTarget/SnipeTarget'
import { useSnipeStore } from './store/snipeTargetStore'
import { useTwitterSnipeStore } from './store/twitterSnipeTargetsStore'

function App() {
  const [dialogs, setDialogs] = useState<DialogData[]>([])
  const snipeTargets = useSnipeStore((state) => state.targets)
  const fetchSnipeTargets = useSnipeStore((state) => state.fetchSnipeTargets)

  const twitterTargets = useTwitterSnipeStore((state) => state.twitterTargets);
  const fetchTwitterTargets = useTwitterSnipeStore((state) => state.fetchTwitterSnipeTargets);

  console.info(twitterTargets)

  useEffect(() => {
    fetchSnipeTargets();
    fetchTwitterTargets();
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
        {/* <TokenBuyPanel onBuy={()=>{}} mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/> */}
        {/* <Token mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/> */}
        <div>
        {twitterTargets.map((target, index) => (
            <SnipeTarget
              key={target.targetName}
              targetId={index}
              isTwitterTarget={true}
              pastShills={[]} // or any real past shills if you have them
              {...target}
            />
          ))}
        </div>
        <div>
          {snipeTargets.map((target) => (
            <SnipeTarget key={target.targetId} {...target} />
          ))}
        </div>
      </div>
    </>
  )
}

export default App
