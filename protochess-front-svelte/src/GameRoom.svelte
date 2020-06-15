<script>
    import WebChess from './Chess/WebChess.svelte';
    import Chat from "./Chat/Chat.svelte";
    import PList from "./PlayersList/PlayersList.svelte";
    import {fly} from 'svelte/transition';
    import {Connected} from "./WebsocketStore";
    import ChessEditor from "./ChessEditor/ChessEditor.svelte";

    let visible = true;

</script>
<style>
    main {
        padding-left: 5em;
        padding-right: 5em;
    }
    h1 {
        margin: 0.2em 0.2em 0.1em;
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 3em;
        font-weight: 100;
    }

    #boardWrapper{
        display: inline-block;
        position: relative;
        top: 0;
        left: 0;
        width: 45em;
        height: 45em;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
    }
    #chatWrapper {
        height: 45em;
        width: 20em;
        background-color: white;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
    }


</style>
<main>
    <h1>Protochess</h1>
    <button on:click={()=> visible = !visible }>Toggle Chat</button>
    {#if $Connected }
        <span style="color: green"> ✓ Connected</span>
    {:else}
        <span style="color: red"> ✖ Disconnected</span>
    {/if}

    <div style="display: inline-block">
        <PList/>
    </div>
    <div style="position: relative;">
        <!-- center board -->
        <div style="width: 100%; text-align: center">
            <div id="boardWrapper">
                <WebChess/>
            </div>
        </div>
        {#if visible}
            <div transition:fly="{{ x: -200, duration: 500 }}" style="position: absolute; top:0; left: 0;" >
                <div id="chatWrapper" style="border:1px solid lightgray;">
                    <Chat/>
                </div>
            </div>
        {/if}
    </div>
</main>
