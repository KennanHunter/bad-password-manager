<script lang="ts">
    import Navbar from "./Components/Navbar.svelte";
    import routes from "./routes.js";
    import Router from "svelte-spa-router";

    // let login: boolean = true;
</script>

<main>
    <Navbar />
    <div>
        <Router {routes} />
        <!-- <h1>Epic</h1> -->
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
    }

    footer {
        background-color: #333;
        padding: 10px;
        color: white;
    }

    /* h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    } */

    div {
        display: grid;
        place-items: center;
        height: 100%;
        flex-grow: 1;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>
