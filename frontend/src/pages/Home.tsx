import { Link } from "react-router-dom";

export const Home = () => {


    return (
        <div>
            <div>
                <h1>Agenda</h1>
            </div>
            <div>
                <div>
                    <Link to="/todo">Todos</Link>
                </div>
                <div>
                    <Link to="/todo">Agenda</Link>
                </div>
                <div>
                    <Link to="/todo">Deadlines</Link>
                </div>
                <div>
                    <Link to="/todo">News</Link>
                </div>
            </div>
        </div>
    )
};
