module.exports = {
	verbose: true,
	moduleFileExtensions: [
		"js",
		"jsx",
		"ts",
		"tsx",
		"json",
	],
	testPathIgnorePatterns: [
		"<rootDir>/src/__tests__/setup/",
		"<rootDir>/src/__tests__/examples/",
		"<rootDir>/cypress",
		"<rootDir>/before",
		"<rootDir>/db/refer",
		"<rootDir>/db/schema",
		"<rootDir>/tests",
	],
};
