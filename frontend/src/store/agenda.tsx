import {createSlice} from '@reduxjs/toolkit';
import {apiCallBegan} from './baseapi';

export type AgendaEntry = {
}

export type NewAgendaEntry = {

}
  
type AgendaState = {
}
 
const agendaSlice = createSlice({
    name: 'agenda',
    initialState:{

    } as AgendaState,
    reducers: {

    }
});

export default agendaSlice.reducer;

const {} = agendaSlice.actions;



