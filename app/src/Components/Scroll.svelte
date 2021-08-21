<script>
    import ScrollItem from "./ScrollItem.svelte";
    export let data;
</script>

<ul>
    {#each data as dat}
        <ScrollItem {dat} />
    {/each}
</ul>

<style>
    ul {
        overflow-y: scroll;
        height: 100%;

        margin: 0;
        padding: 0;
    }
</style>
