<script>
    export let playersList = {
        names:[],
        you: "anon"
    };
</script>

<div>
    <ul style="padding:0">
        {#each playersList.names as player, i}
            <li style="margin-left:0.8em; display:inline-block; color: {playersList.you === player ? 'blue' : 'black'}">
                {i === 0 ? "⭐ " : "- "}{playersList.you === player ? player + " (You)" : player}
            </li>
        {/each}
    </ul>
</div>
