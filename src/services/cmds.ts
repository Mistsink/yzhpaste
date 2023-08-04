import { invoke } from '@tauri-apps/api/tauri'

export interface ImageContent {
  width: number
  height: number
  base64: string
}

export interface RecordImage {
  data_type: 'image'
  content: ImageContent
}

export interface RecordText {
  data_type: 'text'
  content: string
  content_highlight: string | null
  content_preview: string
}

export interface RecordCommon {
  id: number
  md5: string
  active_time: number
  pined: boolean
  tags: string[]
}

export interface RecordOrigin {
  id: number
  md5: string
  active_time: number
  pined: boolean
  tags: string[]
  data_type: 'text' | 'image'
  content: string
  content_highlight: string | null
  content_preview: string
}

export type Record = RecordCommon & (RecordImage | RecordText)

export interface QueryReq {
  key: string | null
  limit: number | null
  pined: boolean | null
  tags: string[] | null
}

interface tmp_clipboard_data {
  text: string
  image: string
}

export const cmd_get_clipboard_data = async (): Promise<tmp_clipboard_data> => {
  return await invoke('get_clipboard_data')
}

export const cmd_clear_data = async (): Promise<boolean> => {
  return await invoke('clear_data')
}

export const cmd_insert_record = async (record: Record): Promise<boolean> => {
  const r = record as RecordOrigin
  if (record.data_type === 'image') r.content = JSON.stringify(record.content)
  return await invoke('insert_record', { r })
}

export const cmd_insert_if_not_exist = async (record: Record): Promise<boolean> => {
  const r = record as RecordOrigin
  if (record.data_type === 'image') r.content = JSON.stringify(record.content)
  return await invoke('insert_if_not_exist', { r })
}

export const cmd_find_all_records = async (): Promise<Record[]> => {
  const records: RecordOrigin[] = await invoke('find_all_records')
  return records.map((r) => {
    if (r.data_type === 'image') r.content = JSON.parse(r.content)
    return r as Record
  })
}

export const cmd_mark_pined = async (id: number): Promise<boolean> => {
  return await invoke('mark_pined', { id })
}

export const cmd_save_tags = async (id: number, tags: string): Promise<boolean> => {
  return await invoke('save_tags', { id, tags })
}

export const cmd_delete_by_id = async (id: number): Promise<boolean> => {
  return await invoke('delete_by_id', { id })
}

export const cmd_find_by_key = async (query: QueryReq): Promise<Record[]> => {
  const records: RecordOrigin[] = await invoke('find_by_key', { query })
  return records.map((r) => {
    if (r.data_type === 'image') r.content = JSON.parse(r.content)
    return r as Record
  })
}

export const cmd_delete_over_limit = async (limit: number): Promise<boolean> => {
  return await invoke('delete_over_limit', { limit })
}

export const cmd_delete_older_than_days = async (days: number): Promise<boolean> => {
  return await invoke('delete_older_than_days', { days })
}

export const cmd_open_window = async () => {
  await invoke('open_window')
}

export const cmd_print = async (msg: string) => {
  console.log('print', msg)
  await invoke('print', { msg })
}

export const cmd_escape_win = async () => {
  await invoke('escape_win')
}

export const cmd_write_to_clip = async (id: number) => {
  await invoke('write_to_clip', { id })
}

export const cmd_paste_in_previous_window = async () => {
  await invoke('paste_in_previous_window')
}

export interface CfgHotkeys {
  active: string
}

export interface CommonConfig {
  version: number
  language: string
  theme_mode: string
  enable_auto_launch: boolean
  enable_auto_paste: boolean
  enable_delete_confirm: boolean
  hotkeys: CfgHotkeys
  record_limit: number
  record_limit_days: number
}

export const cmd_get_common_config = async (): Promise<CommonConfig> => {
  return await invoke('get_common_config')
}

export const cmd_set_common_config = async (config: any) => {
  await invoke('set_common_config', { config })
}

export const cmd_change_languge = async (lang: string) => {
  await invoke('change_languge', { lang })
}

export const cmd_change_record_limit = async (limit: number) => {
  await invoke('change_record_limit', { limit })
}

export const cmd_change_record_limit_days = async (days: number) => {
  await invoke('change_record_limit_days', { days })
}

export const cmd_change_theme_mode = async (mode: string) => {
  await invoke('change_theme_mode', { mode })
}

export const cmd_change_auto_launch = async (enable: boolean) => {
  await invoke('change_auto_launch', { enable })
}

export const cmd_change_auto_paste = async (enable: boolean) => {
  await invoke('change_auto_paste', { enable })
}

export const cmd_change_delete_confirm = async (enable: boolean) => {
  await invoke('change_delete_confirm', { enable })
}

export const cmd_change_hotkey = async (hotkey: CfgHotkeys) => {
  await invoke('change_hotkey', { hotkey })
}