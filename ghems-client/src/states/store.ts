import { configureStore } from "@reduxjs/toolkit";
import { AppReducer } from "./app-state";

export type RootState = ReturnType<typeof store.getState>;

export const store = configureStore({
  reducer: {
    app: AppReducer,
  },
  middleware: (getDefaultMiddleware) => getDefaultMiddleware({ serializableCheck: true }),
  devTools: process.env.NODE_ENV !== 'production',
});

export const appDispatch = store.dispatch;
