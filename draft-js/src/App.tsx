import * as React from "react";
import { render } from "react-dom";
import Draft from "./Draft";

import "./draft.css";

// interface User {
//     name: String
// }

// interface Author {
//     email: String
// }

const user = {
    name: "Steadylearner"
}

const author = {
    email: "example@email.com"
}

function App() {
    return (
        <div className="App">
            <Draft 
                user={user}
                author={author}
            />
        </div>
    );
}

const rootElement = document.getElementById("main");
render(<App />, rootElement);

