<script>
    import {createEventDispatcher} from "svelte";
    const dispatch = createEventDispatcher();
    export let toolSelected;
    export let pieceOwnerSelected;
    export let customPieceCharSelected;
    export let pieceSelected;
    export let registeredChars;
    export let unregisteredChars;

    let PieceType = {
        PAWN: 'p',
        BISHOP: 'b',
        KNIGHT: 'n',
        QUEEN: 'q',
        KING: 'k',
        ROOK: 'r',
        CUSTOM: '#',
    }
    let ToolType = {
        TILE: 1,
        PIECE: 2,
    };
</script>
<style>
    .customCharEntry {
        display: flex;
    }
    .customCharPic{
        width: 3em;
        height: 3em;
    }
    #customCharList {
        width: 10em;
        overflow: auto;
        height: 10vh;
    }
</style>
<div class="field" >
    <label class="label">Select Tool</label>
    <div class="control">
        <label class="radio">
            <input type=radio bind:group={toolSelected} value={ToolType.TILE}/>
            Toggle tiles
        </label>
    </div>
    <div class="control">
        <label class="radio">
            <input type=radio bind:group={toolSelected} value={ToolType.PIECE}/>
            Place piece
        </label>
    </div>
    {#if toolSelected === ToolType.PIECE}
        <hr>
        <label class="label">Tool Options</label>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceOwnerSelected} value={0}/>
                White
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceOwnerSelected} value={1}/>
                Black
            </label>
        </div>
        <hr>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.PAWN}/>
                Pawn
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.KNIGHT}/>
                Knight
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.BISHOP}/>
                Bishop
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.ROOK}/>
                Rook
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.QUEEN}/>
                Queen
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.KING}/>
                King
            </label>
        </div>
        <div class="control">
            <label>
                <input type=radio bind:group={pieceSelected} value={PieceType.CUSTOM}/>
                Custom
            </label>
        </div>

        {#if pieceSelected === PieceType.CUSTOM}
            <div>
                Custom Piece:
                <div id="customCharList">
                    {#each registeredChars as char}
                        <div class=customCharEntry style="background-color:
                            {customPieceCharSelected === char ? 'rgba(0,0,255,0.6)' : 'rgba(0,0,255,0.3)'};"
                             on:click={()=> customPieceCharSelected = char}>
                            <div> Ready </div>
                            <div class="customCharPic">
                                <img src={'/images/chess_pieces/' + (pieceOwnerSelected === 0 ? char.toUpperCase() : char) + '.svg'}/>
                            </div>
                        </div>
                    {/each}

                    {#each unregisteredChars as char}
                        <div class=customCharEntry style="background-color:
                                {customPieceCharSelected === char ? 'rgba(255,00,0,0.6)' :  'rgba(255,0,0,0.3)'};"
                             on:click={()=> customPieceCharSelected = char}>
                            <div> ??? </div>
                            <div class="customCharPic">
                                <img src={pieceOwnerSelected === 0 ? "/images/chess_pieces/white/" + char + ".svg" : "/images/chess_pieces/black/" + char + ".svg"}/>
                            </div>
                        </div>
                    {/each}
                </div>
            </div>
            <button class="button" on:click={()=> dispatch('showPieceEditor')}>
                {registeredChars.includes(customPieceCharSelected) ? 'Edit movement' : 'Set movement'}
            </button>
        {/if}
    {/if}
</div>
