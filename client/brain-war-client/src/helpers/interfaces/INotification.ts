import { ToastType } from "vue3-toastify";


export default interface INotification {
  type: ToastType;
  isRead: boolean;
  content: string;
}
