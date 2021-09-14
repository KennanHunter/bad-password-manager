<script>
	import Scroll from "../Components/Scroll.svelte";
	let username = "Kennan";

	let data = [
		{
			name: "GitHub",
			username: "Kennanhunter",
			website: "github.com",
			password: "password",
		},

		{
			name: "Osu",
			username: "KennanTheGamer",
			website: "osu.ppy.sh",
			password: "password",
		},

		{
			name: "Minecraft",
			username: "Kennnan",
			website: "minecraft.com",
			password: "password",
		},

		{
			name: "Google",
			username: "Kennanhunter5@gmail.com",
			website: "google.com",
			password: "password",
		},

		{
			name: "Something",
			username: "Kennanhunter5@gmail.com",
			website: "something.com",
			password: "password",
		},

		{
			name: "Steam",
			username: "Kennanhunter5@gmail.com",
			website: "steam.com",
			password: "password",
		},
	];
</script>

<div>
	<aside id="scroll">
		<Scroll {data} />
	</aside>
	<h1>{username}'s Vault</h1>
</div>

<style>
	div {
		display: grid;
		width: 100%;
		height: 100%;
		grid-template-columns: min-content auto;
		background-color: white;
		margin: 0;
	}
	aside {
		border: 1px solid black;
		height: 100%;
		overflow-y: auto;
	}
</style>
