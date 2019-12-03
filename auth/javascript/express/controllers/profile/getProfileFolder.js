const getProfileFolder = () => {
	const splitCurrentDirname = __dirname.split("/");
	const payload = splitCurrentDirname.slice(0, splitCurrentDirname.length - 2);
	const profileFolder = `${payload.join("/")}/public/images/profile`;
	return profileFolder;
};

module.exports = getProfileFolder;