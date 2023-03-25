import {createSlice} from '@reduxjs/toolkit';
import {apiCallBegan} from './baseapi';

export type Todo = {
  title: string,
  description: string,
}

type TodosState = {
  todosList: Todo[],
  todosLoading: boolean
}

const todosSlice = createSlice({
  name: 'todos',
  initialState: {
    todosList: [],
    todosLoading: false,
  } as TodosState,

  reducers: {
    todosFetchRequested: (state, action) => {
      state.todosLoading = false;
    },
    todosFetchReceived: (state, action) => {
      state.todosList = action.payload;
    },
    todosFetchFailed: (state, action) => {
      state.todosLoading = false;
    },
  },
});

export default todosSlice.reducer;

const {todosFetchReceived, todosFetchFailed, todosFetchRequested} =
  todosSlice.actions;

export const loadTodos = (forceReload : boolean) => (dispatch: any, getState: any) => {
  const todosList = getState().todos.todosList;
  if (todosList.length === 0 || forceReload === true) {
    return dispatch(
      apiCallBegan({
        url: '/',
        onStart: todosFetchRequested.type,
        onSuccess: todosFetchReceived.type,
        onError: todosFetchFailed.type,
      }),
    );
  }
};
