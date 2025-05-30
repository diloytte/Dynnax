import { useEffect, useState } from 'react'
import './App.css'
import Dialog, { DialogData } from './components/dialog/Dialog'
import SnipeTarget from './components/snipeTarget/SnipeTarget'
import { useSnipeStore } from './store/snipeTargetStore'
import { useTwitterSnipeStore } from './store/twitterSnipeTargetsStore'
import { fetchWithFallback } from './utils'

function App() {
  const [dialogs, setDialogs] = useState<DialogData[]>([])
  const snipeTargets = useSnipeStore((state) => state.targets)
  const fetchSnipeTargets = useSnipeStore((state) => state.fetchSnipeTargets)

  const twitterTargets = useTwitterSnipeStore((state) => state.twitterTargets);
  const fetchTwitterTargets = useTwitterSnipeStore((state) => state.fetchTwitterSnipeTargets);

  useEffect(() => {
    fetchSnipeTargets();
    fetchTwitterTargets();
  }, []);

  useEffect(() => {
    const fetchDialogs = async () => {
      try {
        const res = await fetchWithFallback('/api/v1/tg/dialogs', {
          method: 'GET',
        });
        const data = await res.json();
        if (data.dialogs) {
          setDialogs(data.dialogs);
        }
      } catch (err) {
        console.error('Failed to fetch dialogs:', err);
      }
    };
    fetchDialogs();
  }, []);

  return (
    <>
      <div className='App'>
        <div className=".dialogs">
        {dialogs.map((dialog,index)=>(
          <Dialog id={dialog.id} name={dialog.name} dialogType={dialog.dialogType} isSnipeTarget={false}/>
        ))
        }
        </div>
        {/* <TokenBuyPanel onBuy={()=>{}} mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/> */}
        {/* <Token mint='4W1qX9t4kRSVv8pH4PjkGRpsZ9MNfnxmrUTP8NJZpump' symbol='test' name='Test' logo='https://d23exngyjlavgo.cloudfront.net/solana_6rj9mHEuB52bUTyRUzwV9ZN1buPDHgBYb5zQJi1tpump'/> */}
        <div>
        {twitterTargets.map((target, index) => (
            <SnipeTarget
              key={target.targetName}
              targetId={index}
              isTwitterTarget={true}
              pastShills={[]}
              {...target}
            />
          ))}
        </div>
        <div>
          {snipeTargets.map((target) => (
            <SnipeTarget key={target.targetId} {...target} isTwitterTarget={false} />
          ))}
        </div>
      </div>
    </>
  )
}

export default App
