import { useRef, useMemo } from "react"
import "./BubblesBackground.scss"

export type BubblesBackgroundProps = {
}

export const BubblesBackground = (props: BubblesBackgroundProps) => {
  return <div className="bubbles-background">
    <ul className="circles">
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
      <li></li>
    </ul>
  </div>
}
