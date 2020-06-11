<script>
    export let tile;
    export let gameDimensions;
    export let tileDimensions;
    export let flipped = false;
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

</script>

<div on:click={()=> dispatch('tile_click', tile)}
     style="width:{tileDimensions.width + '%'};
             height:{tileDimensions.height + '%'};
             position: absolute;
             left: {(!flipped ? tile.x : gameDimensions.width - tile.x - 1) * tileDimensions.width + '%'};
             bottom: {(!flipped ? tile.y : gameDimensions.height - tile.y - 1)  * tileDimensions.height + '%'};
             background-color: {tile.tile_type === 'b' ? '#a97d5d' : '#f7dcb4'}">
</div>