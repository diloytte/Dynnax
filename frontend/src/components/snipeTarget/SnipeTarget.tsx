import { updateSnipeTarget } from '../../api/updateSnipeTarget'
import { globalSnipeTargetConfigurations } from '../../constants'
import Dialog from '../dialog/Dialog'
import Slider from '../slider/Slider'
import Toggle from '../toggle/Toggle'
import styles from './SnipeTarget.module.scss'

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
  isTwitterTarget: boolean
}

export interface TwitterSnipeTargetData {
  targetName: string,
  deactiveOnSnipe: boolean,
  isActive: boolean,
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
        isSnipeTarget={true}
        isTwitter={snipeTargetData.isTwitterTarget}
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
      isTwitterTarget: snipeTargetData.isTwitterTarget,
      ...(snipeTargetData.isTwitterTarget
        ? { target_name: snipeTargetData.targetName }
        : { target_id: snipeTargetData.targetId }),
      is_active: value,
    }).catch(console.error)
  }
  name={"Is Active?"}
  initialValue={snipeTargetData.isActive}
/>

<Toggle
  onToggle={(value) =>
    updateSnipeTarget({
      isTwitterTarget: snipeTargetData.isTwitterTarget,
      ...(snipeTargetData.isTwitterTarget
        ? { target_name: snipeTargetData.targetName }
        : { target_id: snipeTargetData.targetId }),
      deactivate_on_snipe: value,
    }).catch(console.error)
  }
  name={"Deactivate on Snipe?"}
  initialValue={snipeTargetData.deactiveOnSnipe}
/>
        </div>
      </div>
    </div>
  )
}

export default SnipeTarget