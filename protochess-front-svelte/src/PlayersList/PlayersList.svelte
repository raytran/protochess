<script>
    import {PlayersList} from '../WebsocketStore';
</script>

<div>
    <ul style="padding:0">
        {#each $PlayersList.names as player, i}
            <li style="margin-left:0.8em; display:inline-block; color: {$PlayersList.you === player ? 'blue' : 'black'}">
                {i === 0 ? "⭐ " : "- "}{$PlayersList.you === player ? player + " (You)" : player}
            </li>
        {/each}
    </ul>
</div>
