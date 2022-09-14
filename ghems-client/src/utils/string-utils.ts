import { CommonUtils } from "./common"

const joinWithSep = (src: string, sep?: string, ...dst: (string | undefined)[]): string => {
  if (!dst) {
    return src
  }

  return [src, ...dst.filter(CommonUtils.identity)].join(sep || " ")
}

const join = (src: string, ...dst: (string | undefined)[]): string => joinWithSep(src, " ", ...dst)

export const StringUtils = {
  join,
  joinWithSep,
}
