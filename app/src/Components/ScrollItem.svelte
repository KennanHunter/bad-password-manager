<script>
    export let dat;
</script>

<li>
    <h2>{dat.name}</h2>
    <h4>Username: {dat.username}</h4>
    <h4>Website: {dat.website}</h4>
</li>

<style>
    li {
        border: 0.5px solid grey;
        list-style-type: none;
        padding: 0;
        margin: 0;
        font-family: "Fira Code", monospace;
    }
</style>
