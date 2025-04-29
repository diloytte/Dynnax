// src/store/snipeStore.ts
import { create } from 'zustand'
import { TwitterSnipeTargetData } from '../components/snipeTarget/SnipeTarget'

interface SnipeStore {
    twitterTargets: TwitterSnipeTargetData[]
    fetchTwitterSnipeTargets: () => Promise<void>
    deleteTwitterTarget: (targetName: String) => Promise<void>
}


export const useTwitterSnipeStore = create<SnipeStore>((set) => ({
    twitterTargets: [],
    
    fetchTwitterSnipeTargets: async () => {
        try {
            const API_KEY = import.meta.env.VITE_API_TOKEN;
            const res = await fetch('http://localhost:8001/api/v1/snipeX', {
                method: 'GET',
                headers: {
                    'Authorization': `Bearer ${API_KEY}`, 
                    'Content-Type': 'application/json'
                }
            });
            const data = await res.json()
            const targets = data.twitter_snipe_targets

            const parsedTargets: TwitterSnipeTargetData[] = Object.entries(targets).map(
                ([targetName, targetRaw]) => {
                    const target = targetRaw as {
                        target_name: string
                        deactivate_on_snipe: boolean
                        is_active: boolean
                        snipe_config: {
                            priority_fee: number
                            slippage: number
                            sol_amount: number
                        }
                    }

                    return {
                        targetName: targetName,
                        deactiveOnSnipe: target.deactivate_on_snipe,
                        isActive: target.is_active,
                        snipeConfig: {
                            priorityFee: target.snipe_config.priority_fee,
                            slippage: target.snipe_config.slippage,
                            solAmount: target.snipe_config.sol_amount,
                        },
                    }
                }
            )

            set({twitterTargets:parsedTargets})
        } catch (err) {
            console.error('Failed to fetch snipe targets:', err)
        }
    },


    deleteTwitterTarget: async (name:String) => {
        const res = await fetch(`http://localhost:8001/api/v1/snipeX/${name}`, {
            method: 'DELETE',
            headers: {
                'Authorization': `Bearer ${import.meta.env.VITE_API_TOKEN}`,
                'Content-Type': 'application/json'
            }
        });
        if (!res.ok) throw new Error('Failed to delete target')

        set((state) => ({
            twitterTargets: state.twitterTargets.filter((t) => t.targetName !== name),
        }))
    },
}))