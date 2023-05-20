import {PayloadAction, createSlice} from '@reduxjs/toolkit';
import {apiCallBegan} from './baseapi';

export type NewsEntry = {
    title: string,
    description: string
}
  
type NewsState = {
    newsLoading : boolean,
    newsList: NewsEntry[]
}
 
const newsSlice = createSlice({
    name: 'news',
    initialState:{
        newsLoading: false,
        newsList: []

    } as NewsState,
    reducers: {
        newsFetchRequested: (state,action ) => {
            state.newsLoading = true;
        },
        newsFetchSuccess: (state,action: PayloadAction<NewsEntry[]>) => {
            state.newsLoading = false;
            state.newsList = action.payload; 
        },
        newsFetchError: (state,action) => {
            state.newsLoading = false;
        }

    }
});

export default newsSlice.reducer;

const {newsFetchError,newsFetchRequested,newsFetchSuccess} = newsSlice.actions;

export const loadNews = (forceReload: boolean = false) => (dispatch: any, getState: any) => {
    const newsList = getState().news.newsList;

    if(newsList.length === 0 || forceReload === true) {
    return dispatch(
        apiCallBegan({
            url: '/scrape',
            onSuccess: newsFetchSuccess.type,
            onStart: newsFetchRequested.type,
            onError: newsFetchError.type
        })
    )
    }
}


