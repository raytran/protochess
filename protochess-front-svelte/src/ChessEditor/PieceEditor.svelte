<script>
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

    import {createEventDispatcher} from 'svelte';
    import {DisplayMode} from "../MovementPatternDisplay/DisplayMode";
    import MovementPatternDisplay from "../MovementPatternDisplay/MovementPatternDisplay.svelte";

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
    #container {
        display: grid;
        padding: 1em;
        column-gap: 1em;
        row-gap: 1em;
        align-items: start;
        justify-items: center;
        width: 100%;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    }

    #mpWrapper {
        width: 100%;
        height: 100%;
    }
    #toolSelector {
        display: grid;
        justify-items: center;
        font-size: 1em;
        grid-template-columns: repeat(auto-fit, minmax(0.5em, 1fr));
    }
    #slideBoxes {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(0.5em, 1fr));
    }

    #leftPanel{
        width: 90%;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        background-color: white;
    }
    #rightPanel{
        width: 90%;
        padding: 1em;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        background-color: white;
    }

    fieldset{
        border: 0;
    }
    fieldset > legend{
        font-weight: bold;
    }
    #wrapper{
        margin: 2em;
        padding: 2em;
        background-color: white;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
    }
    h1{
        text-align: center;
    }

</style>

<div id = wrapper>
    <h1>Piece Editor</h1>
    <div id=container>
        <div id="leftPanel">
            <fieldset>
                <button on:click={saveChanges}>Save Changes</button>
                <button on:click={reset}>Reset</button>
                <hr>
                <label>
                    <input type=checkbox bind:checked = {flipped}>
                    <b>View as black</b>
                </label>
                <hr>
                <b>Viewport Size</b>
                <label>
                    <input type=number bind:value={size} min=1 max=32>
                    <input step=2 type=range bind:value={size} min=1 max=32>
                </label>


                <b>Display Mode</b>
                <label>
                    <input type=radio value={DisplayMode.ALL} bind:group={displayModeSelected}>
                    All
                </label>

                <label>
                    <input type=radio value={DisplayMode.ATTACK} bind:group={displayModeSelected}>
                    Attacks
                </label>

                <label>
                    <input type=radio value={DisplayMode.TRANSLATE} bind:group={displayModeSelected}>
                    Translates
                </label>

            </fieldset>
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
        <div id="rightPanel">
            <div style="text-align: center">
                <h3>Moves</h3>
            </div>
            <fieldset>
                <legend>Delta-Based Moves</legend>
                <div id="toolSelector">
                    <div>
                        Jumps:
                        <label>
                            <input type=radio value={ToolType.ATTACK_JUMP} bind:group={toolSelected}>
                            Attack
                        </label>

                        <label>
                            <input type=radio value={ToolType.TRANSLATE_JUMP} bind:group={toolSelected}>
                            Translate
                        </label>
                    </div>
                    <div>
                        Slides:
                        <label>
                            <input type=radio value={ToolType.ATTACK_SLIDE} bind:group={toolSelected}>
                            Attack Slide
                        </label>
                        {#if toolSelected === ToolType.ATTACK_SLIDE}
                            <button on:click={()=> movementPattern.attackSlideDeltas = [...movementPattern.attackSlideDeltas, []]}>New group</button>
                        {/if}

                        <label>
                            <input type=radio value={ToolType.TRANSLATE_SLIDE} bind:group={toolSelected}>
                            Translate Slide
                        </label>
                        {#if toolSelected === ToolType.TRANSLATE_SLIDE}
                            <button on:click={()=> movementPattern.translateSlideDeltas = [...movementPattern.translateSlideDeltas, []]}>New group</button>
                        {/if}
                    </div>
                </div>
            </fieldset>
            <div id ="slideBoxes">
                <div>
                    <fieldset>
                        <legend>Slide North</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.north}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.north}>
                            Translate
                        </label>
                    </fieldset>
                    <fieldset>
                        <legend>Slide East</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.east}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.east}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide South</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.south}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.south}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide West</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.west}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.west}>
                            Translate
                        </label>
                    </fieldset>
                </div>
                <div>
                    <fieldset>
                        <legend>Slide Northeast</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.northeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.northeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Northwest</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.northwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.northwest}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Southeast</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.southeast}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.southeast}>
                            Translate
                        </label>
                    </fieldset>

                    <fieldset>
                        <legend>Slide Southwest</legend>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.attackSlides.southwest}>
                            Attack
                        </label>
                        <label>
                            <input type=checkbox bind:checked={movementPattern.translateSlides.southwest}>
                            Translate
                        </label>
                    </fieldset>
                </div>
            </div>
        </div>
    </div>
</div>
