import { useEffect, useState } from "react";

import { DataTable } from "primereact/datatable";
import { Column } from "primereact/column";
import { useAppDispatch, useAppSelector } from "../main";
import { NewTodo, deleteTodo, insertTodo, loadTodos } from "../store/todos";
import { Link } from "react-router-dom";
import { Button } from "primereact/button";
import { InputText } from "primereact/inputtext";
import {ConfirmDialog} from "primereact/confirmdialog";

export const Todos = () => {
    const dispatch = useAppDispatch();
    const todosState = useAppSelector((state) => state.todos);
    const [description, setDescription] = useState<string>("");
    const [title, setTitle] = useState<string>("");
    const [id, setId] = useState<number>();
    const { todosList, todosLoading, todosOperationLoading } = todosState;
    const [visible, setVisible] = useState<boolean>(false);

    useEffect(() => {
        dispatch(loadTodos(false));
    }, []);

    const onDeleteClicked = (id: number) => {
        setId(id);
        setVisible(true); 
    }

    const onAddButtonClicked = () => {
        const toAdd = {title, description, visibility: 0} as NewTodo;
        dispatch(insertTodo(toAdd)).then(() => {
            setDescription("");
            setTitle("");
        });
    }

    const buttonDescriptionBody = (data: any) => {
        return (
            <div>
                {data.description}
                <Button onClick={() => onDeleteClicked(data.id)} className="ml-4" severity="danger">Done!</Button>
            </div>
        )
    }

    const onDeleteAccept = () => {
        if(id) {
            dispatch(deleteTodo(id)).then(() => {
                dispatch(loadTodos(true))
            });
        }
    }

    return (
        <div >
            <h1>Todos</h1>
            <div>
                <Link to="/">Home</Link>
            </div>
            <ConfirmDialog visible={visible} onHide={() => setVisible(false)} message="Are you sure you want to complete this todo item?" 
    header="Confirmation" icon="pi pi-exclamation-triangle" accept={onDeleteAccept} />
    

            <DataTable value={todosList} paginator rows={5} loading={todosLoading} >
                <Column className="pb-5 pr-5" field="title" header="Title"></Column>
                <Column
                    className="pb-5 pr-5"
                    field="description"
                    body={buttonDescriptionBody}
                    header="Description"
                ></Column>
            </DataTable>

            <div className="py-5">
                <div className="flex flex-column gap-2 pb-4">
                    <label htmlFor="title">Title</label>
                    <InputText id="title" value={title}
                        onChange={(e) => setTitle(e.target.value)}
                    />
                </div>
                <div className="flex flex-column gap-2 pb-4">
                    <label htmlFor="description">Description</label>
                    <InputText id="description" value={description}
                        onChange={(e) => setDescription(e.target.value)}
                    />
                </div>
                <div className="flex py-4 align-content-center	">
                    <Button loading={todosOperationLoading} onClick={() => onAddButtonClicked()}>Add</Button>
                </div>
            </div>

        <br/>
        <br/>
        <br/>
        <br/>
        <br/>
        </div>
    );
};
