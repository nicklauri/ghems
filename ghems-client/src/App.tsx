import { BrowserRouter, Route, Routes } from "react-router-dom"
import { HomeLayout } from "./layouts/HomeLayout/HomeLayout"
import { LoginLayout } from "./layouts/LoginLayout/LoginLayout"
import { HomePage } from "./pages/Home/Home"
import { LoginPage } from "./pages/Login/Login"
import { Controls } from "./pages/Controls/Controls"
import { CenteredLayout } from "./layouts/CenteredLayout"
import { Upload } from "./pages/Upload/Upload"
import "./global.scss"
import { UserService } from "./services"
import { AppService } from "./services/app.service"
import { RequireLogin } from "./layouts/RequireLogin/RequireLogin"
import { RouteConstants } from "./constants/routes.constants"
import { useEffect } from "react"
import { useSelector } from "react-redux"
import { AppSelector } from "./states/app-state"

function App() {
  const hasToken = UserService.hasToken()
  AppService.setLoggedIn(hasToken)

  const isLoggedIn = useSelector(AppSelector.isLoggedIn)

  useEffect(() => {
    if (isLoggedIn) {
      UserService.getUserInfo().then(resp => {
        if (resp.isSuccess) {
          AppService.login(resp.data)
        }
      })
    }
  }, [isLoggedIn])

  return (
    <BrowserRouter>
      <Routes>
        <Route path={RouteConstants.Root} element={<RequireLogin />}>
          <Route element={<HomeLayout />}>
            <Route path="" element={<HomePage />} />
          </Route>
        </Route>
        <Route path={RouteConstants.Controls} element={<CenteredLayout />}>
          <Route path="" element={<Controls />} />
        </Route>
        <Route path={RouteConstants.Upload} element={<CenteredLayout />}>
          <Route path="" element={<Upload />} />
        </Route>
        <Route path={RouteConstants.Login} element={<LoginLayout />}>
          <Route path="" element={<LoginPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  )
}

export default App
