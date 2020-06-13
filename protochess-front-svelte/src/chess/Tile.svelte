<script>
    export let tile;
    export let gameDimensions;
    export let tileDimensions;
    export let flipped = false;
    export let color;
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

</script>

<div
        on:mouseover={()=> dispatch('tileMouseOver', tile)}
        on:mousedown|preventDefault={()=> dispatch('tileMouseDown', tile)}
        on:mouseup={()=> dispatch('tileMouseUp', tile)}
        on:click={()=> dispatch('tileClick', tile)}
        style="width:{tileDimensions.width + '%'};
                height:{tileDimensions.height + '%'};
                position: absolute;
                left: {(!flipped ? tile.x : gameDimensions.width - tile.x - 1) * tileDimensions.width + '%'};
                bottom: {(!flipped ? tile.y : gameDimensions.height - tile.y - 1)  * tileDimensions.height + '%'};
                background-color: {color}">
</div>