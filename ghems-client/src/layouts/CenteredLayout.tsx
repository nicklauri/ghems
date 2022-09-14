import { Outlet } from "react-router-dom"
import "./layouts.scss"

export const CenteredLayout = () => {
  return <div className="centered-layout">
    <Outlet />
  </div>
}
