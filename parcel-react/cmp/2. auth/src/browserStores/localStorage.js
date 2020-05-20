/* eslint-disable */
// https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API/Using_the_Web_Storage_API

const saveStateToLocalStorage = (name = "state", state = {}) => {
	try {
		const serializedState = JSON.stringify(state);
		localStorage.setItem(name, serializedState);
		return true;
	} catch (error) {
		console.log(error);
		return false;
	}
};

const loadStateFromLocalStorage = (name = "state") => {
	try {
		const serializedState = localStorage.getItem(name);
		if (serializedState === null) {
			return undefined;
		}
		return JSON.parse(serializedState);
	} catch (error) {
		console.log(error);
		return false;
	}
};

// localStorage.key(i) shows key
// localStorage.getItem(localStorage.key(i)) shows item

const removeStateFromLocalStorage = (name = "state") => {
	try {
		localStorage.removeItem(name);
		return true;
	} catch (error) {
		console.log(error);
		return false;
	}
};

const clearLocalStorage = () => {
	try {
		localStorage.clear();
		return true;
	} catch (error) {
		console.log(error);
		return false;
	}
};

const useEachKeyFromLocalStorage = (callback) => {
	try {
		for (let i = 0; i < localStorage.length; i++) {
			callback(localStorage.key(i));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachValueFromLocalStorage = (callback) => {
	try {
		for (let i = 0; i < localStorage.length; i++) {
			callback(localStorage.getItem(localStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachKeyAndValueFromLocalStorage = functionForKey => (functionForValue) => {
	try {
		for (let i = 0; i < localStorage.length; i++) {
			functionForKey(localStorage.key(i));
			functionForValue(localStorage.getItem(localStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const showLocalStateToConsole = () => {
	try {
		for (let i = 0; i < localStorage.length; i++) {
			console.log(`${localStorage.key(i)}: ${localStorage.getItem(localStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

const showLocalStateToConsoleInSequence = () => {
	try {
		for (let i = 0; i < localStorage.length; i++) {
			console.log(`${i}: ${localStorage.getItem(localStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

export {
	loadStateFromLocalStorage,
	saveStateToLocalStorage,
	removeStateFromLocalStorage,
	clearLocalStorage,
	useEachKeyFromLocalStorage,
	useEachValueFromLocalStorage,
	useEachKeyAndValueFromLocalStorage,
	showLocalStateToConsole,
	showLocalStateToConsoleInSequence,
};
