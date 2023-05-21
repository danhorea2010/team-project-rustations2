import { PayloadAction, createSlice } from '@reduxjs/toolkit';
import { apiCallBegan } from './baseapi';
import moment from "moment";

export type AgendaEntry = {
    id: number,
    title: string,
    deadline_date: Date
}

export type NewAgendaEntry = {
    title: string,
    deadline_date: Date,
}

type AgendaState = {
    agendaLoading: boolean,
    agendaList: AgendaEntry[],
}

const agendaSlice = createSlice({
    name: 'agenda',
    initialState: {
        agendaList: [],
        agendaLoading: false,

    } as AgendaState,
    reducers: {
        agendasFetchRequested: (state, action) => {
            state.agendaLoading = true;
        },
        agendasFetchReceived: (state, action: PayloadAction<AgendaEntry[]>) => {
            state.agendaLoading = false;
            state.agendaList = action.payload;
        },
        agendasFetchFailed: (state, action) => {
            state.agendaLoading = false;
        },
        agendasInsertRequested: (state, action) => {
            state.agendaLoading = true;
        },
        agendasInsertSuccess: (state, action: PayloadAction<AgendaEntry[]>) => {
            state.agendaList = state.agendaList.concat(action.payload);
            state.agendaLoading = false;
        },
        agendaInsertFailed: (state, action) => {
            state.agendaLoading = false;
        }


    }
});

export default agendaSlice.reducer;

const { agendaInsertFailed, agendasFetchFailed, agendasFetchReceived, agendasFetchRequested, agendasInsertRequested, agendasInsertSuccess } = agendaSlice.actions;


export const loadAgendas = (forceReload: boolean) => (dispatch: any, getState:any) => {
    const agendaList = getState().agenda.agendaList;
    if(agendaList.length === 0 || forceReload === true) {
        return dispatch(
            apiCallBegan({
                url: "/agenda",
                onStart: agendasFetchRequested.type,
                onSuccess: agendasFetchReceived.type,
                onError: agendasFetchFailed.type
            })
        )
    }
}

export const deleteAgenda = ( id: number) => (dispatch: any, getState: any) => {
    return dispatch(
        apiCallBegan({
            url: `/agenda/${id}`,
            method: 'delete',
        })
    )
};

export const insertAgenda = (agenda: AgendaEntry) => (dispatch: any, getState: any) => {
    let {title, deadline_date} =  agenda;

    const deadline = moment(deadline_date).format("YYYY-MM-DDT00:00:00.000000")
    return dispatch(
        apiCallBegan({
            url: '/agenda',
            method: 'post',
            data: {title, deadline},
            onStart: agendasInsertRequested.type,
            onSuccess: agendasInsertSuccess.type,
            onError: agendaInsertFailed.type,
        })
    )
}

