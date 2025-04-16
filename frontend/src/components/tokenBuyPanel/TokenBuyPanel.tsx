import { useState } from 'react'
import styles from './TokenBuyPanel.module.scss'
import Token, { TokenProps } from '../token/Token'

interface TokenBuyPanelProps extends TokenProps {
  onBuy: (amount: number) => void
}

const TokenBuyPanel = ({ mint, symbol, name, logo, onBuy }: TokenBuyPanelProps) => {
  const [selectedAmount, setSelectedAmount] = useState<number>(10)
  const amounts = [5, 10, 20, 30]

  return (
    <div className={styles.TokenBuyPanel}>
      <Token mint={mint} symbol={symbol} name={name} logo={logo} />

      <div className={styles.amountSelector}>
        {amounts.map((amt) => (
          <button
            key={amt}
            className={selectedAmount === amt ? styles.active : ''}
            onClick={() => setSelectedAmount(amt)}
          >
            {amt} SOL
          </button>
        ))}
      </div>

      <div className={styles.buyButtonWrapper}>
        <button className={styles.buyButton} onClick={() => onBuy(selectedAmount)}>
          Buy
        </button>
      </div>
    </div>
  )
}

export default TokenBuyPanel
