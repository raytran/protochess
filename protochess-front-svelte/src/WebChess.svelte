<!-- A Board with websocketstore bindings -->
<script>
    import Board from "./Chess/Board.svelte";
    import {sendRequest, GameState, PlayersList, MovesFrom} from './WebsocketStore';
    import ChessEditor from "./ChessEditor/ChessEditor.svelte";

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

<div ondragstart="return false;" ondrop="return false;">
    <Board
            {highlighted}
            on:gameRequest={handleGameRequest}
            gameState={$GameState}
            width={$GameState.width}
            height={$GameState.height}
            player_num={$PlayersList.player_num}
            flipped={$PlayersList.player_num !== 0} />
</div>

{#if $GameState.winner}
    <svg style="position:absolute; left: 0; top:0; width: 100%" viewBox="0 0 230 150">
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
