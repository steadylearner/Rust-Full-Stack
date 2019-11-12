// https://johannesklauss.github.io/react-hotkeys-hook/docs-use-hotkeys#example

import React, { useState } from "react";
import ReactDOM from "react-dom";
import { useHotkeys } from "react-hotkeys-hook";

import "./styles.css";

function Test() {
	const [count, setCount] = useState(0);
	useHotkeys("ctrl+k", () => setCount((prevCount) => {
		if (prevCount === 0) {
			return prevCount + 1;
		}
		return prevCount + 2;
	}));

	return (
		<p>
            Pressed {count} times.
		</p>
	);
}

ReactDOM.render(<Test />, document.getElementById("root"));
