import React, { StrictMode } from "react";
import ReactDOM from "react-dom";
import Components from "./components";

import { SteadylearnerCSS as CSS } from "./CSS";

const app = document.getElementById("app");

ReactDOM.render(
    <StrictMode>
        <CSS>
            <Components />
        </CSS>
    </StrictMode>,
    app,
);

export default App;
