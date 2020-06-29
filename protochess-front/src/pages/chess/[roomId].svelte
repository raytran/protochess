<script>
    export let roomId;
    import Tabs from 'svelma/src/components/Tabs/Tabs.svelte';
    import Tab from 'svelma/src/components/Tabs/Tab.svelte';
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
    } from '../../_WebsocketStore';
    import {Connected} from "../../_WebsocketStore";
    import ChessEditor from "../../components/ChessEditor/ChessEditor.svelte";

    joinRoom(roomId);

    function requestEdits(e) {
        console.log(e);
        sendRequest({type: "EditGameState", content: e.detail});
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
</style>

<div id="wrapper">
    <div class="container">
        <button class="btn button is-primary" data-clipboard-text={window.location}>
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
    </div>

    <Tabs position="is-centered" >

        <Tab label="Game">
            <div style="height: 100vh">
            <WebChess/>
            </div>
        </Tab>

        {#if $GameInfo.editable === true}
            <Tab label="Editor">
                <div style="height: 100vh">
                    <ChessEditor on:saveChanges={e => requestEdits(e)}  />
                </div>
            </Tab>
        {/if}
    </Tabs>
</div>
