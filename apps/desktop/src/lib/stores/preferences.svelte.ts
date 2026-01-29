// Preferences Store
// Persists user settings to localStorage

export type AppTheme = 'dark' | 'light' | 'system';

class PreferencesStore {
  theme = $state<AppTheme>('dark');
  animationsEnabled = $state(true);
  soundEnabled = $state(true);
  
  constructor() {
    this.load();
  }

  load() {
    if (typeof localStorage === 'undefined') return;
    const stored = localStorage.getItem('faber-preferences');
    if (stored) {
      try {
        const parsed = JSON.parse(stored);
        this.theme = parsed.theme ?? 'dark';
        this.animationsEnabled = parsed.animationsEnabled ?? true;
        this.soundEnabled = parsed.soundEnabled ?? true;
      } catch (e) {
        console.error('Failed to load preferences', e);
      }
    }
  }

  save() {
    if (typeof localStorage === 'undefined') return;
    localStorage.setItem('faber-preferences', JSON.stringify({
      theme: this.theme,
      animationsEnabled: this.animationsEnabled,
      soundEnabled: this.soundEnabled
    }));
  }
  
  toggleTheme() {
    this.theme = this.theme === 'dark' ? 'light' : 'dark';
    this.save();
  }
}

export const preferences = new PreferencesStore();
