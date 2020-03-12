const randomDigit = (length = 1) => {
	const charSet = "012345";
	return parseInt([...Array(length)].reduce(
		// eslint-disable-next-line no-bitwise
		init => init + charSet[~~(Math.random() * charSet.length)],
		"",
	), 10);
};

export {
	randomDigit,
};
