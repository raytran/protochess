<script>
    export let roomId;
    import WebChess from './_WebChess.svelte';
    import PList from "../../components/PlayersList/PlayersList.svelte";
    import ClipboardJS from "clipboard";
    import { beforeUrlChange } from "@sveltech/routify"
    import {
        sendRequest,
        GameInfo,
        PlayersList,
        MovesFrom,
        leaveRoom,
        CurrentRoom,
        joinRoom
    } from '../../WebsocketStore';
    import {Connected} from "../../WebsocketStore";
    import ChessEditor from "../../components/ChessEditor/ChessEditor.svelte";

    joinRoom(roomId);

    function requestEdits(e) {
        console.log(e);
        sendRequest(
                {
                    type: "EditGameState", content: {
                        width: e.detail.width,
                        height: e.detail.height,
                        tiles: e.detail.tiles,
                        pieces: e.detail.pieces,
                        movement_patterns: e.detail.movement_patterns
                    }
                });
    }

    $beforeUrlChange((event, store) => {
        leaveRoom();
        return true
    });
    new ClipboardJS('.btn');
</script>
<style>
    #wrapper{
        padding-top: 0.5rem;
    }
    h1 {
        margin-top: 0;
        margin-bottom: 0;
    }

    .tabs {
        display: flex;
        flex-wrap: wrap;
        box-shadow: 0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22);
    }

    .tabs label {
        order: 1;
        display: block;
        width: 50%;
        padding-top: 0.8rem;
        text-decoration: underline;
        text-align: center;
        padding-bottom: 0.8rem;
        cursor: pointer;
        background: rgba(0, 0, 0, 0.2);
        font-weight: bold;
        transition: background ease 0.2s;
    }

    .tabs .tab {
        order: 99;
        flex-grow: 1;
        width: 100%;
        display: none;
        padding: 1rem;
        background: #fff;
    }

    .tabs input[type="radio"] {
        display: none;
    }

    .tabs input[type="radio"]:checked + label {
        background: #ffffff;
    }

    .tabs input[type="radio"]:checked + label + .tab {
        display: block;
    }
</style>

<div id="wrapper">
    <button class="btn" data-clipboard-text={window.location}>
        Copy URL to clipboard
    </button>
    {#if $Connected }
        <span style="color: green"> ✓ Connected</span>
    {:else}
        <span style="color: red"> ✖ Disconnected</span>
    {/if}
    <div style="display: inline-block">
        <PList playersList={$PlayersList}/>
    </div>
    <div class="tabs">
        <input type="radio" name="tabs" id="tabone" checked="checked">
        <label for="tabone">Chess</label>
        <div class="tab">
            <WebChess/>
        </div>

        {#if $GameInfo.editable === true}
            <input type="radio" name="tabs" id="tabtwo">
            <label for="tabtwo">Editor</label>
            <div class="tab">
                <ChessEditor on:saveChanges={e => requestEdits(e)}  />
            </div>
        {/if}
    </div>
</div>
