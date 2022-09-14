export const identity = <T>(val: T): T => val
export const ret = <T, E = T>(condition: any | undefined, then: T, orelse?: E): T | E | undefined => Boolean(condition) ? then : orelse
export const merge = <T>(...list: T[]) => list.flat()
export const sortAtoZ = <T>(list: T[], keySelector: (item: T) => string): T[] => list.sort((a, b) => keySelector(a) > keySelector(b) ? 1 : -1)
export const isFn = <T>(obj: T): boolean => typeof obj === "function"

const setPageTitle = (title: string) => document.title = title

export const CommonUtils = {
  identity,
  ret,
  setPageTitle,
  merge,
  sortAtoZ,
}
