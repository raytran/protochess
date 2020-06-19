<script>
    export let piece;
    export let gameWidth;
    export let gameHeight;
    export let flipped = false;

    $: piece_char_rep = piece.owner === 0 ? piece.piece_type.toUpperCase() : piece.piece_type.toLowerCase();
    $: src = "/images/chess_pieces/" + piece_char_rep + ".svg";

</script>
<style>
    img {
        -khtml-user-select: none;
        -o-user-select: none;
        -moz-user-select: none;
        -webkit-user-select: none;
        user-select: none;
    }

    div {
        position: relative;
        text-align: center;
        color: white;
        font-size: 1.2em;
        -webkit-text-stroke-width: 1px;
        -webkit-text-stroke-color: black;
        width: 100%;
        height: 0;
        padding-bottom: 100%;
        pointer-events: none;
        grid-column: var(--x);
        grid-row: var(--y);
    }
    #pieceText {
        position: absolute;
        left: 0;
        top:0;
    }
</style>


{#if piece.x >= 0 && piece.y >= 0 && piece.x < gameWidth && piece.y < gameHeight}
    <div style="--x:{ !flipped ? piece.x + 1 : gameWidth - piece.x };
                --y:{  flipped ? piece.y + 1 : gameHeight - piece.y };">

        <img {src}/>
        {#if piece.piece_text}
        <span id="pieceText">
            {piece.piece_text}
        </span>
        {/if}
    </div>
{/if}
