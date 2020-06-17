<script>
    import WebChess from './WebChess.svelte';
    import Chat from "./Chat/Chat.svelte";
    import PList from "./PlayersList/PlayersList.svelte";
    import {sendRequest, GameState, PlayersList, MovesFrom} from './WebsocketStore';
    import {fly} from 'svelte/transition';
    import {Connected} from "./WebsocketStore";
    import ChessEditor from "./ChessEditor/ChessEditor.svelte";

    let visible = true;

    function requestEdits(e){
        console.log(e);
        sendRequest(
                {type: "EditGameState", content: {
                        width: e.detail.width,
                        height: e.detail.height,
                        tiles: e.detail.tiles,
                        pieces: e.detail.pieces,
                        movement_patterns: e.detail.movement_patterns
                    }
                });
    }
</script>
<style>
    h1 {
        margin: 0.2em 0.2em 0.1em;
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 3em;
        font-weight: 100;
    }

    #boardWrapper{
       margin: 0 auto;
       max-height: 800px;
       max-width: 800px;
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
        <div id="boardWrapper">
                <WebChess/>
        </div>
        {#if visible}
            <div transition:fly="{{ x: -200, duration: 500 }}" style="position: absolute; top:0; left: 0;" >
                <div id="chatWrapper" style="border:1px solid lightgray;">
                    <Chat/>
                </div>
            </div>
        {/if}
    </div>
    <br><br><br><br><br><br><br>
    <div style="display: block">
        <ChessEditor on:saveChanges={e => requestEdits(e)}  />
    </div>

</main>
