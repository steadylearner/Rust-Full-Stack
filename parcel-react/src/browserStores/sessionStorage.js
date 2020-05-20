/* eslint-disable */
// https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API/Using_the_Web_Storage_API

const saveStateToSessionStorage = (name = "state", state = {}) => {
    try {
        const serializedState = JSON.stringify(state);
        sessionStorage.setItem(name, serializedState);
        return true;
    } catch (error) {
        console.log(error);
        return false;
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
        console.log(error);
        return false;
    }
};

// sessionStorage.key(i) shows key
// sessionStorage.getItem(sessionStorage.key(i)) shows item

const removeStateFromSessionStorage = (name = "state") => {
    try {
        sessionStorage.removeItem(name);
        return true;
    } catch (error) {
        console.log(error);
        return false;
    }
};

const clearSessionStorage = () => {
    try {
        sessionStorage.clear();
        return true;
    } catch (error) {
        console.log(error);
        return false;
    }
};

const useEachKeyFromSessionStorage = (callback) => {
    try {
        for (let i = 0; i < sessionStorage.length; i++) {
            callback(sessionStorage.key(i));
        }
    } catch (error) {
        console.log(error);
    }
};

const useEachValueFromSessionStorage = (callback) => {
    try {
        for (let i = 0; i < sessionStorage.length; i++) {
            callback(sessionStorage.getItem(sessionStorage.key(i)));
        }
    } catch (error) {
        console.log(error);
    }
};

const useEachKeyAndValueFromSessionStorage = functionForKey => (functionForValue) => {
    try {
        for (let i = 0; i < sessionStorage.length; i++) {
            functionForKey(sessionStorage.key(i));
            functionForValue(sessionStorage.getItem(sessionStorage.key(i)));
        }
    } catch (error) {
        console.log(error);
    }
};

const showSessionStateToConsole = () => {
    try {
        for (let i = 0; i < sessionStorage.length; i++) {
            console.log(`${sessionStorage.key(i)}: ${sessionStorage.getItem(sessionStorage.key(i))}`);
        }
    } catch (error) {
        console.log(error);
    }
};

const showSessionStateToConsoleInSequence = () => {
    try {
        for (let i = 0; i < sessionStorage.length; i++) {
            console.log(`${i}: ${sessionStorage.getItem(sessionStorage.key(i))}`);
        }
    } catch (error) {
        console.log(error);
    }
};

export {
    loadStateFromSessionStorage,
    saveStateToSessionStorage,
    removeStateFromSessionStorage,
    clearSessionStorage,
    useEachKeyFromSessionStorage,
    useEachValueFromSessionStorage,
    useEachKeyAndValueFromSessionStorage,
    showSessionStateToConsole,
    showSessionStateToConsoleInSequence,
};
