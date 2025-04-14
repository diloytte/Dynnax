import { useEffect, useState } from 'react'
import './App.css'
import Dialog, { DialogData} from './components/dialog/Dialog'

function App() {
  const [dialogs, setDialogs] = useState<DialogData[]>([])

  useEffect(() => {
    const fetchDialogs = async () => {
      try {
        const res = await fetch('http://localhost:8000/api/v1/tg/dialogs')
        const data = await res.json()
        console.log('Fetched dialogs:', data.dialogs)
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
      <div>
      {dialogs.map((dialog) => (
  <Dialog
    key={dialog.id}
    id={dialog.id}
    name={dialog.name}
    dialogType={dialog.dialogType}
  />
))}
    </div>
    </>
  )
}

export default App
