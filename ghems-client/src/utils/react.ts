import { Ref, RefObject } from "react"
import { isFn } from "./common"

const assignRef = <T>(src: T | null = null, dst: Ref<T> | null = null) => {
  if (!dst) return

  if (typeof dst === "function") {
    dst(src)
    return
  }

  (dst as any).current = src
}

export const ReactUtils = {
  assignRef,
}
