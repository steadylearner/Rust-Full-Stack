import React, { useState } from "react";
import PropTypes from "prop-types";

const YupError = ({ children }) => {
	const error = `
		flex
		red margin-bottom-one-and-a-half
		margin-left-two-point font-normal
		hover cursor-pointer errors
	`;
	const [visible, setVisible] = useState(true);

	return (
		<span
			className={visible ? error : error.concat("x-display")}
			onClick={() => { setVisible(false); }}
			title="Pode remover isso com clique caso incomda você."
		>
			{children}
		</span>
	);
};

YupError.propTypes = {
	children: PropTypes.object.isRequired,
};

export {
	YupError,
};
