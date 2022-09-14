
import { ReactNode } from "react"
import "./Dropdown.scss"

export type DropdownProps = {
  options: DropdownItem[],
  searchable?: boolean,
  onChange?: (item: DropdownItem) => void,
}

export type DropdownItem = {
  key: string,
  text: string,
  data?: any,
  onRenderItem?: () => ReactNode,
}

export const Dropdown = (props: DropdownProps) => {

}
