// Console Store
// Centralized log management

export type LogLevel = 'info' | 'warn' | 'error' | 'success' | 'debug';

export interface LogEntry {
  id: string;
  timestamp: Date;
  level: LogLevel;
  message: string;
  source?: string;
}

class ConsoleStore {
  entries = $state<LogEntry[]>([]);
  maxEntries = 1000;

  log(level: LogLevel, message: string, source = 'System') {
    const entry: LogEntry = {
      id: typeof crypto !== 'undefined' && crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(2),
      timestamp: new Date(),
      level,
      message,
      source
    };
    
    this.entries.push(entry);
    
    // Prune
    if (this.entries.length > this.maxEntries) {
      this.entries = this.entries.slice(this.entries.length - this.maxEntries);
    }
  }

  info(msg: string, src?: string) { this.log('info', msg, src); }
  warn(msg: string, src?: string) { this.log('warn', msg, src); }
  error(msg: string, src?: string) { this.log('error', msg, src); }
  success(msg: string, src?: string) { this.log('success', msg, src); }
  debug(msg: string, src?: string) { this.log('debug', msg, src); }
  
  clear() {
    this.entries = [];
  }
}

export const appConsole = new ConsoleStore();
