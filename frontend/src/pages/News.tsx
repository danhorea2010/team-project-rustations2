import { Link } from "react-router-dom"

export const News = () => {

    return (
        <div>
            <h1>News</h1>
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