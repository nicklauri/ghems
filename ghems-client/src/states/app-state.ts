import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Role } from "../models/role.model";
import { EmptyUser, User } from "../models/user.model";
import { RootState } from "./store";

export type AppStateType = {
  title: string,
  isLoggedIn: boolean,
  user: User,
  roles: Role[],
};

const INITIAL_STATE: AppStateType = {
  title: "Ghems",
  isLoggedIn: false,
  user: EmptyUser,
  roles: [],
};

export const AppSlice = createSlice({
  name: "appState",
  initialState: INITIAL_STATE,
  reducers: {
    login(state, user: PayloadAction<User>) {
      state.isLoggedIn = true;
      state.user = user.payload;
    },

    logout(state) {
      state.isLoggedIn = false;
      state.user = EmptyUser;
      state.roles = [];
    },

    setLoggedIn(state, data) {
      state.isLoggedIn = data.payload
    },
  }
});

export const AppActions = AppSlice.actions;
export const AppReducer = AppSlice.reducer;

const selfSelector = (state: RootState) => state.app;

export const AppSelector = {
  title: createSelector(selfSelector, state => state.title),
  isLoggedIn: createSelector(selfSelector, state => state.isLoggedIn),
  user: createSelector(selfSelector, state => state.user),
  role: createSelector(selfSelector, state => state.roles),
}
