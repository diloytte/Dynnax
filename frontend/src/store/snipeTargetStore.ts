// src/store/snipeStore.ts
import { create } from 'zustand'
import { SnipeTargetData } from '../components/snipeTarget/SnipeTarget'
import { fetchWithFallback } from '../utils'

interface SnipeStore {
    targets: SnipeTargetData[]
    fetchSnipeTargets: () => Promise<void>
    deleteTarget: (targetId: number) => Promise<void>
}



export const useSnipeStore = create<SnipeStore>((set) => ({
    targets: [],
    
    fetchSnipeTargets: async () => {
        try {
            const res = await fetchWithFallback('/api/v1/snipe', {
                method: 'GET',
              });
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

            set({targets:parsedTargets})
        } catch (err) {
            console.error('Failed to fetch snipe targets:', err)
        }
    },


    deleteTarget: async (targetId: number) => {
        const res = await fetchWithFallback(`/api/v1/snipe/${targetId}`, {
            method: 'DELETE',
          });
        if (!res.ok) throw new Error('Failed to delete target')

        set((state) => ({
            targets: state.targets.filter((t) => t.targetId !== targetId),
        }))
    },
}))