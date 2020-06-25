<script>
    import {DisplayMode} from "../MovementPatternDisplay/DisplayMode";
    import {createEventDispatcher} from "svelte";
    const dispatch = createEventDispatcher();
    export let flipped;
    export let size;
    export let displayModeSelected;
    export let showSaveChanges = true;
</script>

<div class="field">
    {#if showSaveChanges}
    <button class="button is-danger" on:click={()=> dispatch('reset')}>Reset</button>
    <button class="button is-primary" on:click={()=> dispatch('saveChanges')}>Save Changes</button>
    {/if}
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
