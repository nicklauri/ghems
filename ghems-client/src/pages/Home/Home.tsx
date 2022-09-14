import { useSelector } from "react-redux"
import { Link, Navigate } from "react-router-dom"
import { RouteConstants } from "../../constants/routes.constants"
import { AppSelector } from "../../states/app-state"


export const HomePage = () => {
  return <div className="home-container">
    This is home content.
  </div>
}
