import { User } from "../models/user.model"
import { AppActions } from "../states/app-state"
import { store } from "../states/store"

const setLoggedIn = (state: boolean) => {
  store.dispatch(AppActions.setLoggedIn(state))
}

const isLoggedIn = (): boolean => store.getState().app.isLoggedIn

const login = (user: User) => {
  store.dispatch(AppActions.login(user))
}

const logout = () => {
  store.dispatch(AppActions.logout())
}

export const AppService = {
  setLoggedIn,
  isLoggedIn,
  login,
  logout,
}
