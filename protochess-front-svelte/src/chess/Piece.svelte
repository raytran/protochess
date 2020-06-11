<script>
    import { createEventDispatcher } from 'svelte';
    export let piece;
    export let tileDimensions;
    export let gameDimensions;
    export let flipped = false;
    const dispatch = createEventDispatcher();

    $: piece_char_rep = piece.owner === 0 ? piece.piece_type.toUpperCase() : piece.piece_type.toLowerCase();
    $: src = "/images/chess_pieces/" + piece_char_rep + ".svg";

</script>

<div
        on:click={dispatch('piece_click', piece)}
        style="width:{tileDimensions.width + '%'};
            height:{tileDimensions.height + '%'};
            position: absolute;
            left: {(!flipped ? piece.x : gameDimensions.width - piece.x - 1) * tileDimensions.width + '%'};
            bottom: {(!flipped ? piece.y : gameDimensions.height - piece.y - 1)  * tileDimensions.height + '%'};
">
    <img {src}/>
</div>



