<script>
    import {createEventDispatcher} from "svelte";
    const dispatch = createEventDispatcher();
    export let dimensions;
    export let gameState;
    export let flipped;
    export let gameStateJSON;
    export let showSaveChanges = true;
    import MovementPatternDisplayBar from "../MovementPatternDisplayBar/MovementPatternDisplayBar.svelte";
</script>
<style>
    #movementPatternDisplayBarWrapper {
        height: 20em;
        overflow-y: scroll;
    }
</style>
<div class="field">
    {#if showSaveChanges}
        <button class="button is-danger" on:click={()=> dispatch('reset')}>Reset</button>
        <button class="button is-primary" on:click={()=> dispatch('saveChanges')}>Save Changes</button>
    {/if}
    <div class="control">
        <label class="label">
            <input type=checkbox bind:checked = {flipped}>
            View as black
        </label>
    </div>
    <label class="label">Board Width</label>
    <div class="control">
        <label class="label">
            <input style="max-width: 5rem" type=number bind:value={dimensions.width} min=1 max=16>
            <input type=range bind:value={dimensions.width} min=1 max=16>
        </label>
    </div>
    <label class="label">Board Height</label>
    <div class="control">
        <label>
            <input style="max-width: 5rem" type=number bind:value={dimensions.height} min=1 max=16>
            <input type=range bind:value={dimensions.height} min=1 max=16>
        </label>
    </div>

    <label class="label">JSON</label>
    <label>
        <textarea id="gameStateJSONDisplay" readonly value={gameStateJSON}></textarea>
    </label>

</div>
<b>Registered Movement Patterns:</b>
<div id="movementPatternDisplayBarWrapper">
    <MovementPatternDisplayBar movementPatterns={gameState.movement_patterns} />
</div>
