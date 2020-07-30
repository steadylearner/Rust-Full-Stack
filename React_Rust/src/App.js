import React, { StrictMode } from "react";
import ReactDOM from "react-dom";
import App from "./components";

// const App = () => <ReduxRouter store={store} history={history} />;

const app = document.getElementById("app");

ReactDOM.render(
	<StrictMode><App /></StrictMode>,
	app,
);

export default App;
