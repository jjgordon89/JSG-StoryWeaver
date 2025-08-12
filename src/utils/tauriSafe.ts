// Extend Window interface to include Tauri API
declare global {
  interface Window {
    __TAURI__?: {
      core: {
        invoke: <T = any>(cmd: string, args?: Record<string, any>) => Promise<T>;
      };
      event: {
        listen: <T = any>(event: string, handler: (event: { payload: T }) => void) => Promise<() => void>;
        emit: <T = any>(event: string, payload?: T) => Promise<void>;
      };
    };
  }
}

// Check if we're running in Tauri environment
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;
const isProdBuild = typeof import.meta !== 'undefined' && (import.meta as any).env && Boolean((import.meta as any).env.PROD);

// Get Tauri invoke function if available
const getTauriInvoke = () => {
  if (isTauri && window.__TAURI__) {
    return window.__TAURI__.core.invoke;
  }
  return null;
};

// Get Tauri listen function if available
const getTauriListen = () => {
  if (isTauri && window.__TAURI__) {
    return window.__TAURI__.event.listen;
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
  
  const tauriEmit = window.__TAURI__.event.emit;
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
    // In web browser environment
    if (isProdBuild) {
      // Disallow mock responses in production builds
      throw new Error(`Tauri command '${cmd}' invoked without Tauri in production build. Mock responses are disabled in production.`);
    }
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
      
      case 'auto_write_stream':
      case 'guided_write_stream':
        return { success: true, stream_id: `mock_${Date.now()}` } as T;
      
      case 'auto_write':
      case 'guided_write':
        return {
          generated_text: 'Mock AI generated text for development',
          tokens_used: 25,
          processing_time: 1000,
          context_used: 'Mock context',
          model_used: 'mock-model'
        } as T;

      // Advanced AI mock endpoints for local UI development
      case 'start_streaming_generation':
        return `mock_${Date.now()}` as unknown as T;
      case 'get_stream_status':
        return { status: 'completed', progress: 100, current_text: 'Mock streamed content' } as T;
      case 'cancel_streaming_generation':
        return undefined as T;
      case 'save_generated_content':
        return { success: true } as T;
      
      default:
        // For unknown commands, return empty object
        return {} as T;
    }
  }
  
  return tauriInvoke(cmd, args);
}

/**
 * Check if the application is running in Tauri environment
 * @returns true if running in Tauri, false if in web browser
 */
export function isTauriEnvironment(): boolean {
  return isTauri;
}
