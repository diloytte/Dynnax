import { useEffect, useState } from 'react'
import './App.css'
import Dialog, { DialogData } from './components/dialog/Dialog'
import SnipeTarget, { SnipeTargetData } from './components/snipeTarget/SnipeTarget'

function App() {
  const [dialogs, setDialogs] = useState<DialogData[]>([])
  const [snipeTargets, setSnipeTargets] = useState<SnipeTargetData[]>([])
  const [a,setA] = useState();
  
  useEffect(() => {
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

  useEffect(() => {
    const fetchSnipeTargets = async () => {
      try {
        const res = await fetch('http://localhost:8001/api/v1/snipe')
        const data = await res.json()
        const targets = data.snipe_targets

        const parsedTargets: SnipeTargetData[] = Object.entries(targets).map(
          ([targetId, targetRaw]) => {
            const target = targetRaw as {
              target_name: string
              deactivate_on_snipe: boolean
              is_active: boolean
              past_shills: any[]
              snipe_config: {
                priority_fee: number
                slippage: number
                sol_amount: number
              }
            }
        
            return {
              targetId: Number(targetId),
              targetName: target.target_name,
              deactiveOnSnipe: target.deactivate_on_snipe,
              isActive: target.is_active,
              pastShills: target.past_shills || [],
              snipeConfig: {
                priorityFee: target.snipe_config.priority_fee,
                slippage: target.snipe_config.slippage,
                solAmount: target.snipe_config.sol_amount,
              },
            }
          }
        )
        

        setSnipeTargets(parsedTargets)
      } catch (err) {
        console.error('Failed to fetch snipe targets:', err)
      }
      await fetchTokenMetadata();
    }

    fetchSnipeTargets()
  }, [])

  const fetchTokenMetadata = async () => {
    try {
      const res = await fetch(
        'https://solana-gateway.moralis.io/token/mainnet/SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt/metadata',
        {
          method: 'GET',
          headers: {
            accept: 'application/json',
            'X-API-Key': import.meta.env.VITE_MORALIS_API_KEY, 
          },
        }
      );
  
      if (!res.ok) throw new Error(`Error: ${res.status}`);
  
      const data = await res.json();
      const logoUrl = data.logo;
      setA(logoUrl);

      console.log('Response:', data);
    } catch (err) {
      console.error('Fetch error:', err);
    }
  };

  return (
    <>
      <div className='App'>
          <img src={a}/>
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
