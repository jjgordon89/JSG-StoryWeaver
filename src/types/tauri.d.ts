declare module '@tauri-apps/api' {
  export function invoke(
    command: string,
    params?: any
  ): Promise<any>;
}
