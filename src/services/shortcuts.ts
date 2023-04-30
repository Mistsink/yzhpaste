import {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll,
  type ShortcutHandler
} from '@tauri-apps/api/globalShortcut'
import { getCurrent, appWindow } from '@tauri-apps/api/window'
import { cmd_open_window } from './cmds'

const SCopenWindow = async () => {
  const p = await appWindow.outerPosition()
  await appWindow.setSkipTaskbar(true)
  await register('CmdOrCtrl+Shift+C', async () => {
    await cmd_open_window()
    //     const currentWindow = appWindow.get();
    //   // 获取之前的窗口位置
    //   const currentPosition = await currentWindow.getPosition();
    //   // 打开新窗口
    //   await currentWindow.show();
    //   // 将新窗口移动到之前的位置
    //   await currentWindow.setPosition(currentPosition[0], currentPosition[1]);
    // const curWindow = getCurrent()
    // console.log('reopen', p)
    // await curWindow.show()
    // await curWindow.setPosition(p)
    // await curWindow.setFocus()
    // await curWindow.setSkipTaskbar(true)


  }).catch(console.error)
}

const hideListener = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    await getCurrent().hide().catch(console.error)
  }
}
const handleBeforeUnload = (event: Event) => {
  event.preventDefault()
}

export const useWindowSC = async () => {
  await SCopenWindow()

  window.addEventListener('keydown', hideListener)
  window.addEventListener('beforeunload', handleBeforeUnload)
}
export const unuseWindowSC = async () => {
  await unregisterAll()

  window.removeEventListener('keydown', hideListener)
  window.removeEventListener('beforeunload', handleBeforeUnload)
}
