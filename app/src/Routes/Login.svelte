<script lang="ts">
	import { checkPassStrength } from "../Functions/checkPassStrength";
	import { StoreUsername, ReadUsername } from "../Functions/persistUsername";

	let signupmode = false;

	let passwordInfo;

	interface User {
		username: string;
		password: string;
		remember: boolean;
	}
	let user: User = {
		username: ReadUsername() || "",
		password: "",
		remember: false,
	};

	function submitform() {
		console.log(user);
		console.log(checkPassStrength(user.password));
	}

	function signup() {
		passwordInfo = checkPassStrength(user.password);
		console.log(user);
		console.log(passwordInfo);
	}
</script>

<form name="login">
	<label for="fname">Username</label>
	<input
		type="text"
		id="fname"
		name="username"
		placeholder="Input Username"
		bind:value={user.username}
	/>

	<label for="fpass">Password</label>
	<input
		type="password"
		id="fpass"
		name="password"
		placeholder="Input Password"
		bind:value={user.password}
	/>

	<div id="divremember">
		<label id="rememberlabel" for="fremember">Remember my username</label>
		<input type="checkbox" id="fremember" name="remember" />
	</div>

	{#if (signupmode = true)}
		<p>Your Password is {passwordInfo.text}</p>
	{/if}

	<input type="button" value="Login" id="form-submit" on:click={submitform} />
	<input type="button" value="Sign Up" id="form-submit" on:click={signup} />
</form>

<style>
	form {
		background-color: white;
		border: 2px solid black;
		padding: 30px;
		width: fit-content;
		max-width: 30em;
	}

	input[type="text"],
	input[type="password"] {
		width: 100%;
		padding: 12px 20px;
		margin: 14px 0;
		display: inline-block;
		border: 1px solid rgb(175, 170, 170);
		border-radius: 4px;
		box-sizing: border-box;
	}

	input[type="button"] {
		width: 100%;
		background-color: #3a3a3a;
		color: white;
		padding: 14px 20px;
		margin: 8px 0;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	input[type="button"]:hover,
	input[type="button"]:active {
		background-color: #1f1f1f;
	}
</style>
