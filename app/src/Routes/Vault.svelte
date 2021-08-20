<script>
    import ScrollItem from "../Components/ScrollItem.svelte";
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
    ];
</script>

<div>
    <aside id="scroll">
        <ul>
            {#each data as dat}
                <ScrollItem {dat} />
            {/each}
        </ul>
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
    }
    aside {
        border: 1px solid black;
        overflow-y: auto;
    }
    ul {
        overflow-y: auto;

        margin: 0;
        padding: 0;
    }
</style>
