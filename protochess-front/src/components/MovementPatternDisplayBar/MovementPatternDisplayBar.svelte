<script>
    import MovementPatternDisplay from "../MovementPatternDisplay/MovementPatternDisplay.svelte";
    import {DisplayMode} from "../MovementPatternDisplay/DisplayMode";
    export let flipped = false;
    export let movementPatterns;
    let customPieces = [];
    $: customPieces = (()=>{
        let temp = [];
        if (movementPatterns){
            for (let [pt, mp] of Object.entries(movementPatterns)){
                let size = 7;
                for (let xy of mp.attackJumps){
                    size = Math.max(size, Math.abs(2*xy[0]), Math.abs(2*xy[1]));
                }
                for (let xy of mp.translateJumps){
                    size = Math.max(size, Math.abs(2*xy[0]), Math.abs(2*xy[1]));
                }
                for (let s of mp.attackSlideDeltas){
                    for (let xy of s){
                        size = Math.max(size, Math.abs(2*xy[0]), Math.abs(2*xy[1]));
                    }
                }
                for (let s of mp.translateSlideDeltas){
                    for (let xy of s){
                        size = Math.max(size, Math.abs(2*xy[0]), Math.abs(2*xy[1]));
                    }
                }
                if (size % 2 === 0){
                    size += 1;
                }
                temp.push({pieceType:pt,movementPattern: mp, size:size});
            }
        }
        return temp;
    })();

</script>
<style>
    .mpd{
        margin: 1em;
    }
</style>

<div>
    {#each customPieces as {pieceType, movementPattern, size}}
    <div class="mpd">
        <MovementPatternDisplay
                {movementPattern}
                {pieceType}
                {flipped}
                {size}
                displayMode={DisplayMode.ALL}
        />
    </div>
    {/each}
</div>
