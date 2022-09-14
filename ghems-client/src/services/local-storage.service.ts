import { Option } from "../types/rusty.types"

{
  // Weird hack with localStorage for Firefox:
  const _ = localStorage.length
}

enum StorageKey {
  AuthToken = "auth_token",
}

const getItemRaw = (key: string): Option<string> => localStorage.getItem(key)
const getItem = <T>(key: string): Option<T> => {
  const item = getItemRaw(key)
  if (item) {
    return JSON.parse(item) as T
  }

  return null
}

const isExist = (key: string): boolean => !!getItemRaw(key)

const setItem = <T>(key: string, value: T) => localStorage.setItem(key, JSON.stringify(value))
const removeItem = (key: string) => localStorage.removeItem(key)

const getAuthToken = (): Option<string> => getItem(StorageKey.AuthToken)
const setAuthToken = (token: string) => setItem(StorageKey.AuthToken, token)

export const LocalStorageService = {
  Key: StorageKey,
  getItem,
  getItemRaw,
  getAuthToken,
  setAuthToken,
  isExist,
  setItem,
  removeItem,
}
