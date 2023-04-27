import { invoke } from '@tauri-apps/api/tauri'

export interface Record {
  id: number
  content: string
  content_preview: string | null
  data_type: 'text' | 'image'
  md5: string
  active_time: number
  pined: boolean
  tags: string[]
  content_highlight: string | null
}

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
  return await invoke('insert_record', { r: record })
}

export const cmd_insert_if_not_exist = async (record: Record): Promise<boolean> => {
  return await invoke('insert_if_not_exist', { r: record })
}

export const cmd_find_all_records = async (): Promise<Record[]> => {
  return await invoke('find_all_records')
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

export const cmd_find_by_key = async (query: QueryReq): Promise<[Record]> => {
  return await invoke('find_by_key', { query })
}

export const cmd_delete_over_limit = async (limit: number): Promise<boolean> => {
  return await invoke('delete_over_limit', { limit })
}

export const cmd_delete_older_than_days = async (days: number): Promise<boolean> => {
  return await invoke('delete_older_than_days', { days })
}
