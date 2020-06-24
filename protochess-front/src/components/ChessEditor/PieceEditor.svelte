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
    #toolSelector {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }
    #slideBoxes {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }

    h1{
        text-align: center;
    }

</style>

<div id=container>
    <div class="box" id="leftPanel">
        <div class="field">
            <button class="button is-danger" on:click={reset}>Reset</button>
            <button class="button is-primary" on:click={saveChanges}>Save Changes</button>
            <hr>
            <div class="control">
                <label class="label">
                    <input type=checkbox bind:checked = {flipped}>
                    View as black
                </label>
            </div>
            <hr>
            <label class="label">Viewport Size</label>
            <div class="control">
                <label>
                    <input type=number bind:value={size} min=1 max=32>
                    <input step=2 type=range bind:value={size} min=1 max=32>
                </label>
            </div>


            <label class="label">Display Mode</label>
            <div class="control">
                <label>
                    <input type=radio value={DisplayMode.ALL} bind:group={displayModeSelected}>
                    All
                </label>
            </div>

            <div class="control">
                <label>
                    <input type=radio value={DisplayMode.ATTACK} bind:group={displayModeSelected}>
                    Attacks
                </label>
            </div>

            <div class="control">
                <label>
                    <input type=radio value={DisplayMode.TRANSLATE} bind:group={displayModeSelected}>
                    Translates
                </label>
            </div>
        </div>
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
        <div class="field">
            <label class="label">Delta-Based Moves</label>
            <div id="toolSelector">
                <div class="control">
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
                <div class="control">
                    Slides:
                    <label>
                        <input type=radio value={ToolType.ATTACK_SLIDE} bind:group={toolSelected}>
                        Attack Slide
                    </label>

                    <label>
                        <input type=radio value={ToolType.TRANSLATE_SLIDE} bind:group={toolSelected}>
                        Translate Slide
                    </label>
                    {#if toolSelected === ToolType.ATTACK_SLIDE}
                        <button on:click={()=> movementPattern.attackSlideDeltas = [...movementPattern.attackSlideDeltas, []]}>New group</button>
                    {/if}
                    {#if toolSelected === ToolType.TRANSLATE_SLIDE}
                        <button on:click={()=> movementPattern.translateSlideDeltas = [...movementPattern.translateSlideDeltas, []]}>New group</button>
                    {/if}
                </div>
            </div>
        </div>
        <div id ="slideBoxes">
            <div class="field">
                <label class="label">Slide North</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.north}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.north}>
                        Translate
                    </label>
                </div>
            </div>
            <div class="field">
                <label class="label">Slide East</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.east}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.east}>
                        Translate
                    </label>
                </div>
            </div>

            <div class="field">
                <label class="label">Slide South</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.south}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.south}>
                        Translate
                    </label>
                </div>
            </div>

            <div class="field">
                <label class="label">Slide West</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.west}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.west}>
                        Translate
                    </label>
                </div>
            </div>
            <div class="field">
                <label class="label">Slide Northeast</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.northeast}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.northeast}>
                        Translate
                    </label>
                </div>
            </div>

            <div class="field">
                <label class="label">Slide Northwest</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.northwest}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.northwest}>
                        Translate
                    </label>
                </div>
            </div>

            <div class="field">
                <label class="label">Slide Southeast</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.southeast}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.southeast}>
                        Translate
                    </label>
                </div>
            </div>

            <div class="field">
                <label class="label">Slide Southwest</label>
                <div class="control">
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.attackSlides.southwest}>
                        Attack
                    </label>
                    <label class="checkbox">
                        <input type=checkbox bind:checked={movementPattern.translateSlides.southwest}>
                        Translate
                    </label>
                </div>
            </div>
        </div>
    </div>
</div>
