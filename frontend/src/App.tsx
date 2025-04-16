import { useEffect, useState } from 'react'
import './App.css'
import Dialog, { DialogData } from './components/dialog/Dialog'
import SnipeTarget from './components/snipeTarget/SnipeTarget'
import { useSnipeStore } from './store/snipeTargetStore'

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
