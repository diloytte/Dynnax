import styles from './Token.module.scss'

export interface TokenProps{
    mint:string,
    name:string,
    symbol:string,
    logo:string // url
}

const Token = ({ mint, symbol, name, logo }: TokenProps) => {
    return (
      <div className={styles.Token}>
        <div className={styles.column}>
          <div className={styles.itemWrapper}>
            <img src={logo} alt={name} className={styles.logo} />
          </div>
          <div className={styles.itemWrapper}>
            <span><strong>CA:</strong></span>
            <span>{mint.slice(0,4)}...{mint.slice(-4)}</span>
          </div>
        </div>
        <div className={styles.column}>
          <div className={styles.itemWrapper}>
            <span><strong>Name:</strong></span>
            <span>{name}</span>
          </div>
          <div className={styles.itemWrapper}>
            <span><strong>Symbol:</strong></span>
            <span>{symbol}</span>
          </div>
        </div>
      </div>
    )
  }

export default Token