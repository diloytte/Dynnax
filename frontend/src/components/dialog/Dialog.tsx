import styles from './Dialog.module.scss';
import user from '../../assets/user.png';
import megaphone from '../../assets/megaphone.png';
import privateIcon from '../../assets/private.png';
import imgNotFound from '../../assets/imgnotfound.png';
import trash from '../../assets/delete.png';
import { useSnipeStore } from '../../store/snipeTargetStore';

export enum DialogType {
  User = 0,
  Group = 1,
  Channel = 2
}

export type DialogData = {
  id: number,
  name: string,
  dialogType: DialogType,
  imageUrl?: string;
  isSnipeTarget: boolean
}

const getTypeIcon = (type: DialogType): string => {
  switch (type) {
    case DialogType.User:
      return user;
    case DialogType.Group:
      return privateIcon;
    case DialogType.Channel:
      return megaphone;
    default:
      return '';
  }
};

const Dialog: React.FC<DialogData> = ({ id, name, dialogType, isSnipeTarget }: DialogData) => {
    const deleteTarget = useSnipeStore((state) => state.deleteTarget)
  
  return (
    <div className={styles.dialogCard}>
      <div className={styles.imageContainer}>
        <img
          src={imgNotFound}
          alt='dialogImage'
          className={styles.dialogImage}
        />
      </div>
      <div className={styles.dialogInfo}>
        <div className={styles.basic}>
          <span><strong>ID:</strong> {id}</span>
          <span><strong>NAME:</strong> {name}</span>
        </div>
        {!isSnipeTarget &&
          <img
            src={getTypeIcon(dialogType)}
            alt={`${dialogType} icon`}
            className={`${styles.invertColor} ${styles.typeIcon}`}
          />
        }
        { isSnipeTarget &&
        <img
          onClick={()=>{
            deleteTarget(id)
          }}
          src={trash}
          alt={"delete"}
          className={styles.trashImg}
        />
        }
      </div>
    </div>
  );
};

export default Dialog;
