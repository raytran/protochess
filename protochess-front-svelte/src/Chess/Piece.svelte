<script>
    export let piece;
    export let tileDimensions;
    export let gameDimensions;
    export let flipped = false;

    $: piece_char_rep = piece.owner === 0 ? piece.piece_type.toUpperCase() : piece.piece_type.toLowerCase();
    $: src = "/images/chess_pieces/" + piece_char_rep + ".svg";

</script>
<style>
    img{
        -khtml-user-select: none;
        -o-user-select: none;
        -moz-user-select: none;
        -webkit-user-select: none;
        user-select: none;
    }
</style>

{#if piece.x >= 0 && piece.y >= 0 && piece.x < gameDimensions.width && piece.y < gameDimensions.height}
    <div
            style="width:{tileDimensions.width + '%'};
            height:{tileDimensions.height + '%'};
            position: absolute;
            text-align: center;
            color: purple;
            left: {(!flipped ? piece.x : gameDimensions.width - piece.x - 1) * tileDimensions.width + '%'};
            bottom: {(!flipped ? piece.y : gameDimensions.height - piece.y - 1)  * tileDimensions.height + '%'};
">
        <img style="position: absolute; left: 0; top: 0" {src}/>
        {#if piece.piece_text}
            <svg
                    width="100%"
                    height="100%"
                    viewBox="0 0 300 75"
                    preserveAspectRatio="xMinYMid meet"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
            >
                <text
                        x="70"
                        y="-20"
                        font-size="90"
                        fill="black"
                >{piece.piece_text}</text>
            </svg>
        {/if}
    </div>
{/if}
