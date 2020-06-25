<script>
    import PieceLeftControl from "./PieceLeftControl.svelte";
    import {createEventDispatcher} from 'svelte';
    import {DisplayMode} from "../MovementPatternDisplay/DisplayMode";
    import MovementPatternDisplay from "../MovementPatternDisplay/MovementPatternDisplay.svelte";
    import PieceRightControl from "./PieceRightControl.svelte";
    export let pieceType;
    export let movementPattern = null
    if (movementPattern === null) {
        movementPattern = {
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
        }
    }


    const dispatch = createEventDispatcher();

    const ToolType = {
        ATTACK_JUMP: 0,
        TRANSLATE_JUMP: 1,
        ATTACK_SLIDE: 2,
        TRANSLATE_SLIDE: 3
    }


    let flipped = false;
    let size = 9;
    let center;
    $: center = Math.floor(size / 2);

    function reset() {
        size = 9;
        movementPattern = {
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
        }
    }

    let toolSelected = ToolType.ATTACK_JUMP;
    let displayModeSelected = DisplayMode.ALL;
    let clicked = false;

    function activateTool(tile) {
        let dx = tile.x - center;
        let dy = tile.y - center;
        if (!(tile.x === center && tile.y === center)) {
            switch (toolSelected) {
                case ToolType.ATTACK_JUMP:
                    if (movementPattern.attackJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        movementPattern.attackJumps = [...movementPattern.attackJumps, [dx, dy]];
                    } else {
                        movementPattern.attackJumps = movementPattern.attackJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;
                case ToolType.TRANSLATE_JUMP:
                    if (movementPattern.translateJumps.findIndex(jmp => jmp[0] === dx && jmp[1] === dy) === -1) {
                        movementPattern.translateJumps = [...movementPattern.translateJumps, [dx, dy]];
                    } else {
                        movementPattern.translateJumps = movementPattern.translateJumps.filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                    }
                    break;

                case ToolType.ATTACK_SLIDE:
                    var last_index = movementPattern.attackSlideDeltas.length - 1;
                    if (movementPattern.attackSlideDeltas[last_index].findIndex(jmp => jmp[0] === dx && jmp[1] === dy) !== -1) {
                        movementPattern.attackSlideDeltas[last_index] = movementPattern.attackSlideDeltas[last_index].filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                        break;
                    }

                    //Insert if not removed
                    movementPattern.attackSlideDeltas[movementPattern.attackSlideDeltas.length - 1]
                            = [...movementPattern.attackSlideDeltas[movementPattern.attackSlideDeltas.length - 1], [dx, dy]];
                    break;
                case ToolType.TRANSLATE_SLIDE:
                    var last_index = movementPattern.translateSlideDeltas.length - 1;
                    if (movementPattern.translateSlideDeltas[last_index].findIndex(jmp => jmp[0] === dx && jmp[1] === dy) !== -1) {
                        movementPattern.translateSlideDeltas[last_index] = movementPattern.translateSlideDeltas[last_index].filter(jmp => !(jmp[0] === dx && jmp[1] === dy));
                        break;
                    }

                    //Insert if not removed
                    movementPattern.translateSlideDeltas[movementPattern.translateSlideDeltas.length - 1]
                            = [...movementPattern.translateSlideDeltas[movementPattern.translateSlideDeltas.length - 1], [dx, dy]];
                    break;
            }
        }
    }

    function saveChanges() {
        dispatch('saveChanges', {
            pieceType: pieceType,
            movementPattern: movementPattern
        });
    }


</script>
<style>
    #container{
        display: grid;
        justify-items: center;
        column-gap: 1em;
        row-gap: 1em;
        grid-template-areas: 'left-control board board right-control';
        grid-template-columns: repeat(4, 1fr);
        font-size: 1em;
    }
    @media (max-width: 1200px) {
        #container {
            grid-template-columns: repeat(2, 1fr);
            grid-template-areas:
                    'board        board'
                    'left-control right-control'
        }
    }

    @media (max-width: 650px) {
        #container {
            grid-template-columns: 1fr;
            grid-template-areas:
                    'board'
                    'left-control'
                    'right-control'
        }
    }

    #leftPanel{
        grid-area: left-control;
        width: 100%;
        max-width: 350px;
        padding: 1em;
        text-align: left;
    }

    #mpWrapper {
        width: 100%;
        max-width: 700px;
        grid-area: board;
    }

    #rightPanel {
        grid-area: right-control;
        width: 100%;
        max-width: 350px;
        max-height: 100%;
        padding: 1em;
    }

    h1{
        text-align: center;
    }

</style>

<div id=container>
    <div class="box" id="leftPanel">
        <PieceLeftControl
                bind:flipped={flipped}
                bind:size={size}
                bind:displayModeSelected={displayModeSelected}
                on:saveChanges={saveChanges}
                on:reset={reset}
        />
    </div>
    <div id="mpWrapper" on:mouseleave={()=> clicked = false}>
        <MovementPatternDisplay
                {pieceType}
                {flipped}
                {size}
                displayMode={displayModeSelected}
                {movementPattern}
                on:tileMouseUp={()=> clicked = false}
                on:tileMouseOver={e => (clicked) ?  activateTool(e.detail) : ""}
                        on:tileMouseDown={e => {clicked = true; activateTool(e.detail);}}
                        />
    </div>
    <div class="box" id="rightPanel">
        <PieceRightControl
                bind:toolSelected={toolSelected}
                bind:movementPattern={movementPattern}
        />
    </div>
</div>
