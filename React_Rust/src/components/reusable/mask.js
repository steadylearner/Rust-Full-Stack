const phoneNumberMask = [
	"(",
	/[1-9]/,
	/[1-9]/,
	")",
	" ",
	/\d/,
	/\d/,
	/\d/,
	/\d/,
	/\d/,
	"-",
	/\d/,
	/\d/,
	/\d/,
	/\d/,
];

const cpfMask = [
	/\d/,
	/\d/,
	/\d/,
	".",
	/\d/,
	/\d/,
	/\d/,
	".",
	/\d/,
	/\d/,
	/\d/,
	"-",
	/\d/,
	/\d/,
];

export {
	phoneNumberMask,
	cpfMask,
};
