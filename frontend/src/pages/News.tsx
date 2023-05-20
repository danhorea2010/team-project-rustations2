import { Link } from "react-router-dom"
import { useAppDispatch, useAppSelector } from "../main"
import {DataTable} from "primereact/datatable";
import {Column} from "primereact/column";
import { useEffect } from "react";
import { loadNews } from "../store/news";


export const News = () => {

    const dispatch = useAppDispatch();
    const newsState = useAppSelector(state => state.news);
    const {newsList, newsLoading} = newsState;

    useEffect(() => {
        dispatch(loadNews());
    }, [dispatch])


    return (
        <div>
            
            <h1>News</h1>
            {newsLoading && <h2>Loading...</h2>}
            <DataTable value={newsList}>
                <Column className="pb-5 pr-5" field="title" header="Title"></Column>
                <Column className="pb-5 pr-5" field="description" header="Content"></Column>
            </DataTable>
            <div>
                <Link to="/todo">Todos</Link>
            </div>
            <div>
                <Link to="/agenda">Agenda</Link>
            </div>
            <div>
                <Link to="/deadlines">Deadlines</Link>
            </div>
            <div>
                <Link to="/news">News</Link>
            </div>
        </div>
    )
}