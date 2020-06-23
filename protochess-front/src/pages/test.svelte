<script>
    import {onMount} from "svelte";
    import wasm from "../../../protochess-engine-wasm/Cargo.toml"
    import Board from "../components/Chess/Board.svelte";

    let gameState = null;
    let maxTimeout = 3;
    let engineStatus = 'Loading...';
    let myEngine = null;

    onMount(async () => {
        let wasmer = await wasm();
        let engine = wasmer.Protochess;
        myEngine = new engine();
        engineStatus = "Your turn";
    });

    let highlighted = {};
    function handleGameRequest(req) {
        highlighted = {};
        if (req.detail.type === "MovesFrom"){
            let from = [req.detail.content[0], req.detail.content[1]];
            let moves = myEngine.moves_from(req.detail.content[0], req.detail.content[1]);
            highlighted = {in_check_kings: null, possibleMoves: {from: from, to: moves}, lastTurn: null};
        }
        if (req.detail.type === "TakeTurn"){
            let from = req.detail.content.from;
            let to = req.detail.content.to;
            myEngine.make_move(from[0], from[1], to[0], to[1]);
            gameState = myEngine.get_state();
            engineStatus = "Engine thinking...";
            setTimeout(function(){
                let move_res = myEngine.get_best_move_timeout(maxTimeout);
                let move = move_res[0];
                let depth = move_res[1];
                myEngine.make_move(move[0], move[1], move[2], move[3]);
                gameState = myEngine.get_state();
                highlighted.lastTurn = {from: [move[0], move[1]], to:[move[2], move[3]]};
                engineStatus = "Looked "+ depth + " ply. Your turn!";
            }, 500);

        }
        console.log(req);
    }

</script>
<style>
    #boardWrapper {
        max-width: 800px;
    }
    #engineStats {

    }
    progress {
        appearance: none;
        display: block;
        width: 100%;
        height: 1rem;
        border: 0;
    }

    progress::-webkit-progress-bar {
        background: lightgray;
    }

    progress::-webkit-progress-value {
        background: #06c;
        transition: width 2.5s ease-in-out;
    }
</style>

<div id="engineStats">
    {engineStatus}
</div>
<label>
    <input type="number" bind:value={maxTimeout}/>
    Max thinking time (seconds)
</label>

<div id="boardWrapper" ondragstart="return false;" ondrop="return false;">
    <Board
            {highlighted}
            on:gameRequest={handleGameRequest}
            {gameState}
            player_num={0}
            flipped={false} />

</div>

