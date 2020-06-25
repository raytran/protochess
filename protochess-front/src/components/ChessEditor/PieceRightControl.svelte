<script>
    export let toolSelected;
    export let movementPattern;
    const ToolType = {
        ATTACK_JUMP: 0,
        TRANSLATE_JUMP: 1,
        ATTACK_SLIDE: 2,
        TRANSLATE_SLIDE: 3
    }
</script>
<style>
    #toolSelector {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }
    #slideBoxes {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    }
</style>
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
