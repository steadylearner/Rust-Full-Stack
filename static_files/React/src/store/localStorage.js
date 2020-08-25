// https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API/Using_the_Web_Storage_API
// Podemos usar isso direitamente em vez de Redux quando tem pouca necessidade para isso

const saveStateToLocalStorage = (name = "state", state = {}) => {
	try {
		const serializedState = JSON.stringify(state);
		localStorage.setItem(name, serializedState);
	} catch (error) {
		console.log(error);
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
		return undefined;
	}
};

// localStorage.key(i) shows key
// localStorage.getItem(localStorage.key(i)) shows item

const removeStateFromLocalStorage = (name = "state") => {
	try {
		localStorage.getItem(name);
	} catch (error) {
		console.log(error);
	}
};

const useEachKeyFromLocalStorage = (callback) => {
	try {
		for (var i = 0; i < localStorage.length; i++) {
			callback(localStorage.key(i));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachValueFromLocalStorage = (callback) => {
	try {
		for (var i = 0; i < localStorage.length; i++) {
			callback(localStorage.getItem(localStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const useEachKeyAndValueFromLocalStorage = (functionForKey) => (functionForValue) => {
	try {
		for (var i = 0; i < localStorage.length; i++) {
			functionForKey(localStorage.key(i));
			functionForValue(localStorage.getItem(localStorage.key(i)));
		}
	} catch (error) {
		console.log(error);
	}
};

const showLocalStateToConsole = () => {
	try {
		for (var i = 0; i < localStorage.length; i++) {
			console.log(`${localStorage.key(i)}: ${localStorage.getItem(localStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

const showLocalStateToConsoleInSequence = () => {
	try {
		for (var i = 0; i < localStorage.length; i++) {
			console.log(`${i}: ${localStorage.getItem(localStorage.key(i))}`);
		}
	} catch (error) {
		console.log(error);
	}
};

const clearLocalStorage = () => {
	try {
		localStorage.clear();
	} catch (error) {
		console.log(error);
	}
};

export {
	loadStateFromLocalStorage,
	saveStateToLocalStorage,
	removeStateFromLocalStorage,
	useEachKeyFromLocalStorage,
	useEachValueFromLocalStorage,
	useEachKeyAndValueFromLocalStorage,
	showLocalStateToConsole,
	showLocalStateToConsoleInSequence,
	clearLocalStorage,
};

