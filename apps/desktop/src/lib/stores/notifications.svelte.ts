// Notification Store
// Manages global toasts

export type NotificationType = 'info' | 'success' | 'warning' | 'error';

export interface Notification {
  id: string;
  type: NotificationType;
  message: string;
  duration?: number; // ms
}

class NotificationStore {
  items = $state<Notification[]>([]);

  add(type: NotificationType, message: string, duration = 3000) {
    const id = crypto.randomUUID();
    const notification = { id, type, message, duration };
    this.items.push(notification);

    if (duration > 0) {
      setTimeout(() => {
        this.remove(id);
      }, duration);
    }
  }

  remove(id: string) {
    this.items = this.items.filter(n => n.id !== id);
  }

  success(msg: string) { this.add('success', msg); }
  error(msg: string) { this.add('error', msg); }
  info(msg: string) { this.add('info', msg); }
  warning(msg: string) { this.add('warning', msg); }
}

export const notifications = new NotificationStore();
