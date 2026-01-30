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

export interface LogSession {
  id: string;
  name: string;
  entries: LogEntry[];
}

class ConsoleStore {
  sessions = $state<LogSession[]>([
    { id: 'main', name: 'Terminal', entries: [] }
  ]);
  activeSessionId = $state('main');
  maxEntries = 1000;

  get entries() {
    return this.sessions.find(s => s.id === this.activeSessionId)?.entries || [];
  }

  log(level: LogLevel, message: string, source = 'System', sessionId?: string) {
    const targetSessionId = sessionId || this.activeSessionId;
    const session = this.sessions.find(s => s.id === targetSessionId);
    
    if (!session) return;

    const entry: LogEntry = {
      id: typeof crypto !== 'undefined' && crypto.randomUUID ? crypto.randomUUID() : Math.random().toString(36).substring(2),
      timestamp: new Date(),
      level,
      message,
      source
    };
    
    session.entries.push(entry);
    
    // Prune
    if (session.entries.length > this.maxEntries) {
      session.entries = session.entries.slice(session.entries.length - this.maxEntries);
    }
  }

  createSession(name: string = 'Terminal') {
    const id = Math.random().toString(36).substring(2, 9);
    this.sessions.push({ id, name, entries: [] });
    this.activeSessionId = id;
    return id;
  }

  closeSession(id: string) {
    if (this.sessions.length <= 1) return; // Keep at least one
    const index = this.sessions.findIndex(s => s.id === id);
    if (index === -1) return;
    
    this.sessions.splice(index, 1);
    if (this.activeSessionId === id) {
      this.activeSessionId = this.sessions[Math.max(0, index - 1)].id;
    }
  }

  info(msg: string, src?: string, sid?: string) { this.log('info', msg, src, sid); }
  warn(msg: string, src?: string, sid?: string) { this.log('warn', msg, src, sid); }
  error(msg: string, src?: string, sid?: string) { this.log('error', msg, src, sid); }
  success(msg: string, src?: string, sid?: string) { this.log('success', msg, src, sid); }
  debug(msg: string, src?: string, sid?: string) { this.log('debug', msg, src, sid); }
  
  clear(sid?: string) {
    const targetSessionId = sid || this.activeSessionId;
    const session = this.sessions.find(s => s.id === targetSessionId);
    if (session) session.entries = [];
  }
}

export const appConsole = new ConsoleStore();
