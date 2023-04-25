declare global {
  interface Window {
    __TAURI__?: any // 或者使用更具体的类型，而不是 'any'
  }
}
