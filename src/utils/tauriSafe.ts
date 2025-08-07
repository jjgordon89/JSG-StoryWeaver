// Check if we're running in Tauri environment
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Get Tauri invoke function if available
const getTauriInvoke = () => {
  if (isTauri && window.__TAURI__) {
    return (window as any).__TAURI__.core.invoke;
  }
  return null;
};

// Get Tauri listen function if available
const getTauriListen = () => {
  if (isTauri && window.__TAURI__) {
    return (window as any).__TAURI__.event.listen;
  }
  return null;
};

/**
 * Safe wrapper for Tauri's listen function that handles web browser environments
 * @param event - The event name to listen for
 * @param handler - The event handler function
 * @returns Promise that resolves to an unlisten function or null
 */
export async function listen<T = any>(event: string, handler: (event: { payload: T }) => void): Promise<(() => void) | null> {
  const tauriListen = getTauriListen();
  
  if (!tauriListen) {
    console.warn(`Tauri listen for event '${event}' called in web environment. This is expected during development.`);
    // Return a no-op unlisten function for web environment
    return () => {};
  }
  
  return tauriListen(event, handler);
}

/**
 * Safe wrapper for Tauri's emit function that handles web browser environments
 * @param event - The event name to emit
 * @param payload - The event payload
 * @returns Promise that resolves when the event is emitted
 */
export async function emit<T = any>(event: string, payload?: T): Promise<void> {
  if (!isTauri || !window.__TAURI__) {
    console.warn(`Tauri emit for event '${event}' called in web environment. This is expected during development.`);
    return Promise.resolve();
  }
  
  const tauriEmit = (window as any).__TAURI__.event.emit;
  return tauriEmit(event, payload);
}

/**
 * Safe wrapper for Tauri's invoke function that handles web browser environments
 * @param cmd - The command to invoke
 * @param args - Arguments for the command
 * @returns Promise that resolves to the command result or rejects with an error
 */
export async function invoke<T = any>(cmd: string, args?: Record<string, any>): Promise<T> {
  const tauriInvoke = getTauriInvoke();
  
  if (!tauriInvoke) {
    // In web browser environment, return a mock response or throw an appropriate error
    console.warn(`Tauri command '${cmd}' called in web environment. This is expected during development.`);
    
    // For development purposes, return a mock response based on the command
    switch (cmd) {
      case 'save_document':
      case 'update_document':
      case 'delete_document':
      case 'create_document':
        return {} as T;
      
      case 'get_documents':
        return [] as T;
      
      case 'sync_settings':
      case 'set_setting':
        return undefined as T;
      
      default:
        // For unknown commands, return empty object or throw error based on needs
        return {} as T;
    }
  }
  
  return tauriInvoke<T>(cmd, args);
}

/**
 * Check if the application is running in Tauri environment
 * @returns true if running in Tauri, false if in web browser
 */
export function isTauriEnvironment(): boolean {
  return isTauri;
}