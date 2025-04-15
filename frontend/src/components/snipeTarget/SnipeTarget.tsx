import { updateSnipeTarget } from '../../api/updateSnipeTarget'
import Dialog, { DialogData } from '../dialog/Dialog'
import Slider from '../slider/Slider'
import Toggle from '../toggle/Toggle'
import styles from './SnipeTarget.module.scss'

export const globalSnipeTargetConfigurations = {
  slippage: {
    min: 1,
    max: 100,
  },
  priorityFee: {
    min: 0.005,
    max: 1,
  },
  solAmount: {
    min: 0.5,
    max: 30
  }
}

export interface SnipeTargetData {
  targetId: number,
  targetName: string,
  deactiveOnSnipe: boolean,
  isActive: boolean,
  pastShills: any[] | undefined
  snipeConfig: {
    priorityFee: number,
    slippage: number,
    solAmount: number
  }
}

const SnipeTarget = (snipeTargetData: SnipeTargetData) => {
  return (
    <div className={styles.snipeTarget}>
      <Dialog
        id={snipeTargetData.targetId}
        name={snipeTargetData.targetName}
        dialogType={0}
      />
      <div className={styles.configurations}>
        <div className={styles.sliderWrapper}>
          <Slider
            name="Slippage"
            min={globalSnipeTargetConfigurations.slippage.min}
            max={globalSnipeTargetConfigurations.slippage.max}
            initialValue={snipeTargetData.snipeConfig.slippage}
            sniperTargetId={snipeTargetData.targetId} />
        </div>
        <div className={styles.sliderWrapper}>
          <Slider
            name="Priority Fee"
            min={globalSnipeTargetConfigurations.priorityFee.min}
            max={globalSnipeTargetConfigurations.priorityFee.max}
            initialValue={snipeTargetData.snipeConfig.priorityFee}
            sniperTargetId={snipeTargetData.targetId} />
        </div>
        <div className={styles.sliderWrapper}>
          <Slider
            name="Sol Amount"
            min={globalSnipeTargetConfigurations.solAmount.min}
            max={globalSnipeTargetConfigurations.solAmount.max}
            initialValue={snipeTargetData.snipeConfig.solAmount}
            sniperTargetId={snipeTargetData.targetId} />
        </div>
        <div className={styles.togglesWrapper}>
          <Toggle
            onToggle={(value) =>
              updateSnipeTarget({
                target_id: snipeTargetData.targetId,
                is_active: value,
              }).catch(console.error)}

            name={"is Active?"} initialValue={snipeTargetData.isActive} />
          <Toggle
            onToggle={(value) =>
              updateSnipeTarget({
                target_id: snipeTargetData.targetId,
                deactive_on_snipe: value,
              }).catch(console.error)
            }
            name={"DOS"} initialValue={snipeTargetData.deactiveOnSnipe} />
        </div>
      </div>
    </div>
  )
}

export default SnipeTarget