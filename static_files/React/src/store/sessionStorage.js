// Podemos usar isso em vez de Redux quando tem pouca necessidade para isso

const saveStateToSessionStorage = (name = "state", state = {}) => {
	try {
		const serializedState = JSON.stringify(state);
		sessionStorage.setItem(name, serializedState);
	} catch (error) {
		console.log(error);
	}
};

const loadStateFromSessionStorage = (name = "state") => {
	try {
		const serializedState = sessionStorage.getItem(name);
		if (serializedState === null) {
			return undefined;
		}
		return JSON.parse(serializedState);
	} catch (error) {
		return undefined;
	}
};

const removeStateFromSessionStorage = (name = "state") => {
	try {
		sessionStorage.getItem(name);
	} catch (error) {
		console.log(error);
	}
};

const useEachKeyFromSessionStorage = (callback) => {
	try {
		for (var i = 0; i < sessionStorage.length; i++) {
			callback(sessionStorage.key(i));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachValueFromSessionStorage = (callback) => {
	try {
		for (var i = 0; i < sessionStorage.length; i++) {
			callback(sessionStorage.getItem(sessionStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachKeyAndValueFromSessionStorage = (functionForKey) => (functionForValue) => {
	try {
		for (var i = 0; i < sessionStorage.length; i++) {
			functionForKey(sessionStorage.key(i));
			functionForValue(sessionStorage.getItem(sessionStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const showSessionStateToConsole = () => {
	try {
		for (var i = 0; i < sessionStorage.length; i++) {
			console.log(`${sessionStorage.key(i)}: ${sessionStorage.getItem(sessionStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

const showSessionStateToConsoleInSequence = () => {
	try {
		for (var i = 0; i < sessionStorage.length; i++) {
			console.log(`${i}: ${sessionStorage.getItem(sessionStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

const clearSessionStorage = () => {
	try {
		sessionStorage.clear();
	} catch (error) {
		console.log(error);
	}
};

export {
	loadStateFromSessionStorage,
	saveStateToSessionStorage,
	removeStateFromSessionStorage,
	useEachKeyFromSessionStorage,
	useEachValueFromSessionStorage,
	useEachKeyAndValueFromSessionStorage,
	showSessionStateToConsole,
	showSessionStateToConsoleInSequence,
	clearSessionStorage,
}




