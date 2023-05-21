import { Link } from "react-router-dom";

export const Home = () => {


    return (
        <div>
            <div>
                <h1>Agenda home</h1>
            </div>
            <div>
                <div>
                    <Link to="/todo">Todos</Link>
                </div>
                <div>
                    <Link to="/agenda">Agenda</Link>
                </div>
                <div>
                    <Link to="/news">News</Link>
                </div>
            </div>
        </div>
    )
};
