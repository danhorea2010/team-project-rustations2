import { Link } from "react-router-dom"
import { useAppDispatch, useAppSelector } from "../main"
import {Calendar} from "primereact/calendar"
import { useEffect, useState } from "react";

import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";
import { NewAgendaEntry, deleteAgenda, insertAgenda, loadAgendas } from "../store/agenda";
import {Button} from "primereact/button";
import { InputText } from "primereact/inputtext";
import moment from "moment";

const formatDate = (date: Date) => {
    return moment(date).format("D/M/YYYY");
}

export const Agenda = () => {

    const dispatch = useAppDispatch();
    const agendaState = useAppSelector(state => state.agenda);
     
    const [title, setTitle] = useState<string>("");
    const [deadline, setDeadline] = useState<Date | null>();

    const {agendaList, agendaLoading} = agendaState;

    useEffect(() => {
        dispatch(loadAgendas(false));
    }, [dispatch])

    const onAddButtonClicked = () => {
        const toAdd = {title, deadline_date: deadline} as NewAgendaEntry;

        dispatch(insertAgenda(toAdd)).then(() => {
            setDeadline(null);
            setTitle("");
        })

    }

    const onDelete = (id: number) => {
        dispatch(deleteAgenda(id)).then(() => {
            dispatch(loadAgendas(true))
        });
    }

    const buttonDateBody = (data: any) => {
        return (
            <div>
                {formatDate(data.deadline)}
                <Button onClick={() => onDelete(data.id) } >Remove</Button>
            </div>
        )
    }

    return (
        <div>
            <h1>Agenda</h1>
            <div>
                <Link to="/todo">Todos</Link>
            </div>
            <div>
                <Link to="/agenda">Agenda</Link>
            </div>
            <div>
                <Link to="/news">News</Link>
            </div>

            <DataTable value={agendaList} >
                <Column className="pb-5 pr-5" field="title" header="Title"/>
                <Column className="pb-5 pr-5" body={buttonDateBody} field="deadline" header="Deadline"/>
            </DataTable>

             <div className="flex flex-column gap-2 pb-4">
                    <label htmlFor="deadline">Deadline</label>
                    <Calendar value={deadline} onChange={(e) => setDeadline(e.value as Date | undefined )}/>
                </div>
             <div className="flex flex-column gap-2 pb-4">
                    <label htmlFor="title">Description</label>
                    <InputText id="title" value={title}
                        onChange={(e) => setTitle(e.target.value)}
                    />
                </div>
            <Button loading={agendaLoading} onClick={onAddButtonClicked}>Add</Button>
        </div>
    )
}