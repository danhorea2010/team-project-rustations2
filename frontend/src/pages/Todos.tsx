import { useEffect, useState } from "react";

import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";
import { useAppDispatch, useAppSelector } from "../main";
import { deleteTodo, loadTodos } from "../store/todos";
import { Link } from "react-router-dom";
import {Button} from "primereact/button";

export const Todos = () => {
    const dispatch = useAppDispatch();
    const todosState = useAppSelector((state) => state.todos);
    const { todosList } = todosState;

    useEffect(() => {
        dispatch(loadTodos(false));
    }, []);

    const onDeleteClicked = (id: number) => {
        dispatch(deleteTodo(id)).then(() => {
            dispatch(loadTodos(true))
        });
    }

    const buttonDescriptionBody = (data: any) => {
        return (
            <div>
                {data.description}
                <Button onClick={() => onDeleteClicked(data.id)} className="ml-4" severity="danger">Delete</Button>
            </div>
        )
    }

    return (
        <div>
            <h1>Todos</h1>
            <div>
                <Link to="/">Home</Link>
            </div>

            <DataTable value={todosList} paginator rows={10}>
                <Column className="pb-5 pr-5" field="title" header="Title"></Column>
                <Column
                    className="pb-5 pr-5"
                    field="description"
                    body={buttonDescriptionBody}
                    header="Description"
                ></Column>
            </DataTable>


        </div>
    );
};
