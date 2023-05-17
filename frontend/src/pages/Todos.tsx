import { useEffect, useState } from "react";

import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";
import { useAppDispatch, useAppSelector } from "../main";
import { loadTodos } from "../store/todos";
import { Link } from "react-router-dom";

export const Todos = () => {
    const dispatch = useAppDispatch();
    const todosState = useAppSelector((state) => state.todos);
    const { todosList } = todosState;

    useEffect(() => {
        dispatch(loadTodos(false));
    }, []);

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
                    header="Description"
                ></Column>
                <Column
                    className="pb-5 pr-5"
                    field="visibility"
                    header="Visibility"
                ></Column>
            </DataTable>


        </div>
    );
};
