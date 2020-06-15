<script>
    import MovementPatternDisplay from "../MovementPatternDisplay/MovementPatternDisplay.svelte";
    import {DisplayMode} from "../MovementPatternDisplay/DisplayMode";
    export let customPieces = [
        {pieceType: 'p', movementPattern: {
                attackSlides: {
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                    northeast: false,
                    northwest: false,
                    southeast: false,
                    southwest: false,
                },
                translateSlides: {
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                    northeast: false,
                    northwest: false,
                    southeast: false,
                    southwest: false,
                },
                attackJumps: [],
                translateJumps: [],
                attackSlideDeltas: [[]],
                translateSlideDeltas: [[]]
            }}
    ];

</script>

<div style="display: flex">
    {#each customPieces as {pieceType, movementPattern}}
        <MovementPatternDisplay
                {movementPattern}
                {pieceType}
                flipped={false}
                size={9}
                displayMode={DisplayMode.ALL}
                />
    {/each}
</div>
