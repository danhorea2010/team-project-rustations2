import {createSlice} from '@reduxjs/toolkit';

export const baseApi = createSlice({
  name: 'baseapi',
  initialState: {},
  reducers: {
    apiCallBegan: (state, action) => {},
    apiCallSucess: (state, action) => {},
    apiCallFailed: (state, action) => {},
  },
});

export const {apiCallBegan, apiCallFailed, apiCallSucess} = baseApi.actions;
export default baseApi.reducer;
