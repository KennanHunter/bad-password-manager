<script lang="ts">
	import Navbar from "./Components/Navbar.svelte";
	import routes from "./routes.js";
	import Router from "svelte-spa-router";
</script>

<main>
	<Navbar />
	<div>
		<Router {routes} />
	</div>
	<footer>Made By Kennan Hunter</footer>
</main>

<style>
	main {
		display: grid;
		height: 100%;
		text-align: center;
		padding: 0;
		margin: 0;
		height: 100%;
		/* max-width: 240px; */
		grid-template-rows: auto 85% auto;
		overflow: hidden;
	}

	footer {
		background-color: #333;
		padding: 10px;
		color: white;
		z-index: 10;
	}

	div {
		display: grid;
		place-items: center;
		height: 100%;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
