/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface OpenedList {
  entries: Array<Entry>
}
export interface Entry {
  folderUri?: string
  label?: string
  workspace?: Workspace
  fileUri?: string
}
export interface Workspace {
  id: string
  configPath: string
}
export function getOpenedList(path: string): OpenedList
