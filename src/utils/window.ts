import { open } from '@tauri-apps/api/window';

export async function openClipboardPopup() {
  await open({
    url: 'path/to/your/clipboard-history.html',
    title: 'Clipboard History',
    width: 400,
    height: 300,
    resizable: false,
    fullscreen: false,
    frameless: false,
  });
}
