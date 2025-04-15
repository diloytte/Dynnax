import React, { useState } from "react";
import styles from "./Toggle.module.scss";
import playIcon from "../../assets/play.png";
import pauseIcon from "../../assets/pause.png";

export interface ToggleProps {
  initialValue: boolean
  name?: string,
  onToggle: (value: boolean) => Promise<void>;
}

const Toggle: React.FC<ToggleProps> = ({ initialValue, name, onToggle }: ToggleProps) => {
  const [isActive, setIsActive] = useState(initialValue);

  const toggle = async () => {
    const newState = !isActive;
  
    try {
      await onToggle(newState); 
      setIsActive(newState); 
    } catch (err) {
      console.error("Toggle failed:", err);
    }
  };
  
  return (
    <div className={styles.toggleWrapper}>
      <span>{name}</span>
      <button
        className={`${styles.toggleButton} ${isActive ? styles.active : styles.inactive
          }`}
        onClick={toggle}
      >
        <img
          src={isActive ? playIcon : pauseIcon}
          alt={isActive ? "Pause" : "Play"}
          className={`${styles.icon} invertColor`}
        />
      </button>
    </div>
  );
};

export default Toggle;
