// // Redux configuração, aqui não precisamos

// import { createStore, applyMiddleware, compose } from "redux";
// import { createEpicMiddleware } from "redux-observable";

// // import thunk from "redux-thunk"; // I don't use it anymore
// import { throttle } from "lodash";
// import { createLogger } from "redux-logger";
// import { createBrowserHistory } from "history";
// import { routerMiddleware } from "connected-react-router";

// import rootReducer, { rootEpic } from "../reducers";
// import { saveStateToSessionStorage } from "./sessionStorage";
// // import { saveStateToSessionStorage , loadStateFromSessionStorage } from "./localStorage";

// // to sync react-rotuer-dom with redux

// // react dev -> more for html and css, redux -> more for js and app development
// // https://github.com/zalmoxisus/redux-devtools-extension#12-advanced-store-setup
// // -> Use it for production also?
// // https://codeburst.io/redux-devtools-for-dummies-74566c597d7 - manual for redux dev

// // This line and import { compose } from "redux" makes browser devtool of redux and react work
// const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;

// export const history = createBrowserHistory(); // does order matters here?
// const routerHistoryForRedux = routerMiddleware(history);
// const epicMdware = createEpicMiddleware();
// const loggerMdware = createLogger({
// 	predicate: () => process.env.NODE_ENV === "development",
// });

// const pairs = [];

// // Is it necessary? thunk should be first(later routerHistoryFOrRedux)it helps dispatch multiple actions asyncronously,
// // It also helps to avoid race condition

// // order is important here
// pairs.push(routerHistoryForRedux);
// // pairs.push(thunk);
// pairs.push(epicMdware);
// pairs.push(loggerMdware); // logger should be last

// const configureStore = () => {
// 	// const persistedState = loadStateFromSessionStorage();

// 	const store = createStore(
// 		rootReducer(history),
// 		// persistedState,
// 		composeEnhancers(
// 			applyMiddleware(...pairs),
// 			// applyMiddleware(
// 			// 	routerMiddleware(history),
// 			// 	// thunk,
// 			// 	epicMdware,
// 			// 	loggerMdware,
// 			// ),
// 		),
// 	);

// 	epicMdware.run(rootEpic);

// 	store.subscribe(throttle(() => {
// 		saveStateToSessionStorage(store.getState());
// 		// saveState(store.getState().videosPage);
// 	}, 1000));

// 	// console.log("persisted state: ", persistedState);

// 	return store;
// };

// export default configureStore;
