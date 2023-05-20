import { PayloadAction, createSlice } from '@reduxjs/toolkit';
import { apiCallBegan } from './baseapi';

export type Todo = {
  id: number,
  title: string,
  description: string,
}

type TodosState = {
  todosList: Todo[],
  todosLoading: boolean,
  todosOperationLoading: boolean
}

const todosSlice = createSlice({
  name: 'todos',
  initialState: {
    todosList: [],
    todosLoading: false,
    todosOperationLoading: false,
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
    todosDeleteRequested: (state, action) => {
      state.todosOperationLoading = true;
    },
    todosDeleteSuccess: (state, action) => {
      state.todosOperationLoading = false;
    },
    todosDeleteFailed: (state, action) => {
      state.todosOperationLoading = false;
    },
    todosInsertRequested: (state, action) => {
      state.todosOperationLoading = true;
    },
    todosInsertSuccess: (state, action: PayloadAction<Todo>) => {
      state.todosOperationLoading = false;
      state.todosList.push(action.payload);
    },
    todosInsertFailed: (state, action) => {
      state.todosOperationLoading = false;
    }
  },
});

export default todosSlice.reducer;

const { todosFetchReceived, todosFetchFailed, todosFetchRequested, todosDeleteFailed, todosDeleteSuccess, todosDeleteRequested, todosInsertFailed, todosInsertRequested, todosInsertSuccess } =
  todosSlice.actions;

export const loadTodos = (forceReload: boolean) => (dispatch: any, getState: any) => {
  const todosList = getState().todos.todosList;
  if (todosList.length === 0 || forceReload === true) {
    return dispatch(
      apiCallBegan({
        url: '/todo',
        onStart: todosFetchRequested.type,
        onSuccess: todosFetchReceived.type,
        onError: todosFetchFailed.type,
      }),
    );
  }
};

export const deleteTodo = (id: number) => (dispatch: any, getState: any) => {
  return dispatch( 
    apiCallBegan({
    url: `/todo/${id}`,
    method: 'delete',
    onStart: todosDeleteRequested.type,
    onSuccess: todosDeleteSuccess.type,
    onError: todosDeleteFailed.type,
    }))
}