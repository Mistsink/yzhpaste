import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export const useClipboardStore = defineStore('clipboard', () => {
  const histories = ref<Array<any>>()

  return { histories }
})

const fetchClipboardData = async (): Promise<any> => {
  try {
    const clipboardData = await invoke('get_clipboard_data')
    return JSON.parse(clipboardData as string)
    // 处理剪贴板数据
  } catch (err) {
    console.error('Error fetching clipboard data:', err)
  }
}

const updateClipboardStore = async () => {
    const clipboardStore = useClipboardStore()
    const ctn = await fetchClipboardData()
    // console.log(`ctn ${ctn}`)
    console.log(ctn)
    // clipboardStore.histories = [ctn, clipboardStore.histories]
}

let intervalId = -1
export const startLoop = (ms: number) => {
    if (intervalId !== -1) return
    console.log('start interval')
    intervalId = setInterval(updateClipboardStore, ms)
    console.log(`intervalId: ${intervalId}`)
}

export const stopLoop = () => {
    if (intervalId === -1) return
    clearInterval(intervalId)
}