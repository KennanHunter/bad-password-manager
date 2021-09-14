function StoreUsername(username: string) {
	let store: Storage;
	try {
		store = window.localStorage;
	} catch (err) {
		console.error(err);
		return;
	}
	store.setItem("username", username);
}

function ReadUsername(): string {
	return window.localStorage.getItem("username");
}

export { StoreUsername, ReadUsername };
