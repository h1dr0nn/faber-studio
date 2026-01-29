// App Store
// Manages global app state

class AppStore {
  isBusy = $state(false);
  busyMessage = $state('');
  
  setBusy(busy: boolean, message = '') {
    this.isBusy = busy;
    this.busyMessage = message;
  }
}

export const appState = new AppStore();
