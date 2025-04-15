import React, { useState } from "react";
import styles from "./Slider.module.scss";
import { updateSnipeTarget } from "../../api/updateSnipeTarget";

interface SliderProps {
  min: number;
  max: number;
  name: string;
  sniperTargetId: number;
  initialValue: number;
}

const paramMap: { [key: string]: string } = {
  "Slippage": "slippage",
  "Priority Fee": "priority_fee",
  "Sol Amount": "sol_amount",
};

const Slider: React.FC<SliderProps> = ({
  min,
  max,
  name,
  sniperTargetId,
  initialValue,
}: SliderProps) => {
  const [value, setValue] = useState(initialValue); 
  const [tempValue, setTempValue] = useState(initialValue); // Temporary value for sliding

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = Number(e.target.value);
    setTempValue(newValue); // Update temporary value during sliding
  };

  const handleMouseUp = async () => {
    if (tempValue !== value) { // Check if value changed
      await updateValue(tempValue); 
      setValue(tempValue); // Update the actual value after mouseup
    }
  };

  const handleTouchEnd = async () => {
    if (tempValue !== value) { // Check if value changed
      await updateValue(tempValue); 
      setValue(tempValue); // Update the actual value after touchend
    }
  };

  const updateValue = async (newValue: number) => {
    try {
      const paramName = paramMap[name];

      if (!paramName) {
        console.error("Invalid slider name:", name);
        return;
      }

      const result = await updateSnipeTarget({
        target_id: sniperTargetId,
        [paramName]: newValue,
      });

      console.log("Updated server with value:", newValue);
    } catch (err) {
      console.error("Slider update failed:", err);
    }
  };

  return (
    <div className={styles.sliderContainer}>
      <span className={styles.sliderName}>{name}</span>
      <input
        type="range"
        min={min}
        max={max}
        step={paramMap[name] === "slippage" ? 1 : 0.001} // or whatever precision you want
        value={tempValue} // Bind temporary value here
        onChange={handleChange}
        onMouseUp={handleMouseUp}
        onTouchEnd={handleTouchEnd}
        className={styles.slider}
      />
      <div className={styles.valueDisplay}>{tempValue}</div>
    </div>
  );
};

export default Slider;
