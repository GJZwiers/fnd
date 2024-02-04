import { ReactNode } from "react";
import classes from "./Modal.module.css";

function Modal({
  children,
  onClose,
}: {
  children: ReactNode;
  onClose: () => void;
}) {
  return (
    <>
      <div className={classes.backdrop} onClick={onClose} />
      <dialog open className={classes.modal}>
        {children}
      </dialog>
    </>
  );
}

export default Modal;
