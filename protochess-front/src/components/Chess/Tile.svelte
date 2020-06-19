<script>
    export let tile;
    export let gameWidth;
    export let gameHeight;
    export let flipped = false;
    export let color;
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

</script>
<style>
    div {
        width: 100%;
        height: 0;
        padding-bottom: 100%;
        grid-column: var(--x);
        grid-row: var(--y);
    }
</style>

<div

        style="--x:{ !flipped ? tile.x + 1 : gameWidth - tile.x };
               --y:{  flipped ? tile.y + 1 : gameHeight - tile.y };
                background-color: {color}"
        on:mouseover={()=> dispatch('tileMouseOver', tile)}
        on:mousedown|preventDefault={()=> dispatch('tileMouseDown', tile)}
        on:mouseup={()=> dispatch('tileMouseUp', tile)}
        on:click={()=> dispatch('tileClick', tile)}>
</div>


<!--
                position: absolute;
                left: {(!flipped ? tile.x : gameWidth - tile.x - 1) * tileDimensions.width + '%'};
                bottom: {(!flipped ? tile.y : gameHeight.height - tile.y - 1)  * tileDimensions.height + '%'};
-->