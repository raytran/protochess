<!-- A Board with websocketstore bindings -->
<script>
    import Board from "../../components/Chess/Board.svelte";
    import {sendRequest, GameState, PlayersList, MovesFrom} from '../../WebsocketStore';
    import ChessEditor from "../../components/ChessEditor/ChessEditor.svelte";
    import {fly, fade} from 'svelte/transition';
    import MovementPatternDisplayBar from "../../components/MovementPatternDisplayBar/MovementPatternDisplayBar.svelte";
    import Chat from "../../components/Chat/Chat.svelte";
    import WebChat from "../_WebChat.svelte";

    let chatVisible = true;
    let mpVisible = true;
    let mpFlipped = false;
    let highlighted = {};
    MovesFrom.subscribe(function (e) {
        highlighted.lastTurn = null;
        highlighted.possibleMoves = e;
    });

    //Reset highlighting whenever the gamestate updates
    GameState.subscribe(function (e) {
        highlighted = null;
        highlighted = {in_check_kings: e.in_check_kings, possibleMoves: null, lastTurn: e.last_turn};
    });

    function handleGameRequest(e) {
        sendRequest(e.detail);
    }

</script>


<style>
    #boardWrapper{
        grid-area: board;
        position: relative;
        width: 100%;
        max-width: 700px;
    }

    #chatWrapper {
        grid-area: chat;
        width: 100%;
        max-width: 400px;
        height: 50vh;
        background-color: white;
        -webkit-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        -moz-box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
        box-shadow: 0px 15px 20px -8px rgba(0,0,0,0.4);
    }
    #movementPatternDisplayBarWrapper {
        grid-area: movement-pattern;
        text-align: center;
        height: 50vh;
        width: 100%;
        max-width:400px;
        overflow: scroll;
        box-shadow: 0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22);
    }

    #boardAndMP {
        display: grid;
        justify-items: center;
        column-gap: 1em;
        row-gap: 1em;
        grid-template-columns: repeat(4,  1fr);
        grid-template-areas: 'chat board board movement-pattern';
    }

    @media (max-width: 1200px) {
        #boardAndMP {
            grid-template-columns: repeat(2,  1fr);
            grid-template-areas:
                    'board        board'
                    'chat         movement-pattern'
        }
    }

    @media (max-width: 650px) {
        #boardAndMP {
            grid-template-columns: 1fr;
            grid-template-areas:
                    'board'
                    'chat'
                    'movement-pattern'
        }
    }
</style>
<button on:click={()=> chatVisible = !chatVisible }>Toggle Chat</button>
<button on:click={()=> mpVisible = !mpVisible }>Toggle Movement Pattern</button>
<div  id="boardAndMP">
    {#if chatVisible}
        <div transition:fade="{{duration: 200}}" id="chatWrapper" style="border:1px solid lightgray;">
            <WebChat/>
        </div>
    {/if}
        <!-- center board -->
    <div id="boardWrapper">
        {#if $GameState.winner}
            <svg style="width: 100%" viewBox="0 0 230 150">
                <rect x="0" y="40%" width="100%" height="45%" fill="rgba(30,220,30)" fill-opacity="0.3"/>
                <text x="50%" y="55%"
                      font-family="Arial, Helvetica, sans-serif"
                      dominant-baseline="central" text-anchor="middle" fill="white">
                    {$GameState.winner}
                </text>

                <text x="50%" y="70%"
                      font-family="Arial, Helvetica, sans-serif"
                      dominant-baseline="central" text-anchor="middle" fill="white">
                    WINS!
                </text>
            </svg>
        {/if}
        <div ondragstart="return false;" ondrop="return false;">
            <Board
                    {highlighted}
                    on:gameRequest={handleGameRequest}
                    gameState={$GameState}
                    player_num={$PlayersList.player_num}
                    flipped={$PlayersList.player_num !== 0} />

        </div>

    </div>

    {#if mpVisible}
        <div transition:fade="{{duration: 200}}" id="movementPatternDisplayBarWrapper">
            <div style="border-bottom: 1px solid #eee">
                <h1>Movement Patterns</h1>
                <button on:click={()=> mpFlipped = !mpFlipped }>Toggle Perspective</button>
            </div>
            <MovementPatternDisplayBar flipped={mpFlipped} movementPatterns={$GameState.movement_patterns} />
        </div>
    {/if}
</div>










