import {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll,
  type ShortcutHandler
} from '@tauri-apps/api/globalShortcut'
import { getCurrent, appWindow } from '@tauri-apps/api/window'
import { cmd_escape_win, cmd_open_window, cmd_write_to_clip } from './cmds'

export const useEnterAndEsc = async (id: number) => {
  await cmd_write_to_clip(id)
  await cmd_escape_win()
}

const hideListener = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    await cmd_escape_win()
  }
}
const handleBeforeUnload = (event: Event) => {
  event.preventDefault()
}

export const useWindowSC = async () => {
  window.addEventListener('keydown', hideListener)
  window.addEventListener('beforeunload', handleBeforeUnload)
}
export const unuseWindowSC = async () => {
  await unregisterAll()

  window.removeEventListener('keydown', hideListener)
  window.removeEventListener('beforeunload', handleBeforeUnload)
}
