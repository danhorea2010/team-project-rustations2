import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import 'primeflex/primeflex.css'
import thunk from "redux-thunk"
import {Provider, TypedUseSelectorHook, useDispatch, useSelector} from 'react-redux';
import {configureStore} from '@reduxjs/toolkit';
import api from './store/middleware/api'
import todos from './store/todos'

export const store = configureStore ({
  reducer: {
    todos    
  },
  middleware: [thunk, api]
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch

type DispatchFunc = () => AppDispatch
export const useAppDispatch: DispatchFunc = useDispatch
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector

const AppRedux = () => (
  <Provider store={store}>
    <App />
  </Provider>
);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <AppRedux/>
  </React.StrictMode>,
)
