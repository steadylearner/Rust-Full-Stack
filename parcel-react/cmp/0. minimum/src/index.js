import React from "react";
import ReactDOM from "react-dom";

import HelloMessage from "./HelloMessage";

const app = document.getElementById("app");
ReactDOM.render(<HelloMessage name="Jane" />, app);
