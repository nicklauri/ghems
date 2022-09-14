import { useSelector } from "react-redux"
import { Outlet, useNavigate } from "react-router-dom"
import { RouteConstants } from "../../constants/routes.constants"
import { AppSelector } from "../../states/app-state"

export const RequireLogin = () => {
  const isLoggedIn = useSelector(AppSelector.isLoggedIn)
  const navigate = useNavigate()

  if (!isLoggedIn) {
    navigate(RouteConstants.Login)
  }

  return <Outlet />
}
