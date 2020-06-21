<script>
    import Board from "../Chess/Board.svelte";
    import {createEventDispatcher} from "svelte";
    export let gameState;
    export let isPublic = true;
    export let allowEdits = true;
    const dispatch = createEventDispatcher();
</script>
<style>
    #wrapper {
        padding-top: 0.3em;
        box-shadow: 0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22);
        width: 100%;
        text-align:center;
    }
    #content {
        padding: 1em;
        display: grid;
        grid-gap: 1em;
    }
    #pubPrivSelector {
        display: flex;
        justify-content:space-evenly;
    }

    #pubPrivSelector > label {
        order: 1;
        display: block;
        width: 50%;
        padding-top: 0.8rem;
        text-align: center;
        padding-bottom: 0.8rem;
        cursor: pointer;
        background: rgba(0, 0, 0, 0.2);
        font-weight: bold;
    }

</style>

<div id="wrapper">
    <h1>Create a new room</h1>
    <div id="content">
        <div id="pubPrivSelector">
            <label>
                <input type="radio" bind:group={isPublic} value={true}>
                Public
            </label>
            <label>
                <input type="radio" bind:group={isPublic} value={false}>
                Private
            </label>
        </div>
        <label>
            <input type="checkbox" bind:checked={allowEdits}>
            Allow edits throughout the game?
        </label>

        <Board {gameState}/>
        <button on:click={()=>dispatch('editBoard')}>Edit Board</button>
        <button on:click={()=>dispatch('createRoom')}>Create room</button>

    </div>
</div>