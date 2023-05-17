import {createSlice} from '@reduxjs/toolkit';
import {apiCallBegan} from './baseapi';

export type NewsEntry = {
}
  
type NewsState = {
}
 
const newsSlice = createSlice({
    name: 'news',
    initialState:{

    } as NewsState,
    reducers: {

    }
});

export default newsSlice.reducer;

const {} = newsSlice.actions;

