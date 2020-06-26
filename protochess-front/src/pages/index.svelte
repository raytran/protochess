<script>
    import {RoomList, createRoom} from '../WebsocketStore';
    import Tabs from 'svelma/src/components/Tabs/Tabs.svelte';
    import Tab from 'svelma/src/components/Tabs/Tab.svelte';
    import Button from 'svelma/src/components/Button.svelte';
    import Modal from 'svelma/src/components/Modal/Modal.svelte';
    import MediaQuery from "../components/MediaQuery.svelte";
    import { url, redirect } from '@sveltech/routify'
    import ProtochessInfo from './_ProtochessInfo.svelte';
    import PlayButtons from './_PlayButtons.svelte';
    import FullWidthModal from '../components/Modal.svelte';
    import Rl from "../components/RoomList/RoomList.svelte";
    import CreateRoomDialog from "../components/CreateRoomDialog/CreateRoomDialog.svelte";
    import ChessEditor from "../components/ChessEditor/ChessEditor.svelte";
    let active = false
    let showBoardEditor = false;
    let rawData = JSON.parse("{\"type\":\"GameState\",\"content\":{\"width\":8,\"height\":8,\"winner\":null,\"to_move\":0,\"to_move_in_check\":false,\"in_check_kings\":null,\"last_turn\":null,\"tiles\":[{\"x\":0,\"y\":0,\"tile_type\":\"b\"},{\"x\":0,\"y\":1,\"tile_type\":\"w\"},{\"x\":0,\"y\":2,\"tile_type\":\"b\"},{\"x\":0,\"y\":3,\"tile_type\":\"w\"},{\"x\":0,\"y\":4,\"tile_type\":\"b\"},{\"x\":0,\"y\":5,\"tile_type\":\"w\"},{\"x\":0,\"y\":6,\"tile_type\":\"b\"},{\"x\":0,\"y\":7,\"tile_type\":\"w\"},{\"x\":1,\"y\":0,\"tile_type\":\"w\"},{\"x\":1,\"y\":1,\"tile_type\":\"b\"},{\"x\":1,\"y\":2,\"tile_type\":\"w\"},{\"x\":1,\"y\":3,\"tile_type\":\"b\"},{\"x\":1,\"y\":4,\"tile_type\":\"w\"},{\"x\":1,\"y\":5,\"tile_type\":\"b\"},{\"x\":1,\"y\":6,\"tile_type\":\"w\"},{\"x\":1,\"y\":7,\"tile_type\":\"b\"},{\"x\":2,\"y\":0,\"tile_type\":\"b\"},{\"x\":2,\"y\":1,\"tile_type\":\"w\"},{\"x\":2,\"y\":2,\"tile_type\":\"b\"},{\"x\":2,\"y\":3,\"tile_type\":\"w\"},{\"x\":2,\"y\":4,\"tile_type\":\"b\"},{\"x\":2,\"y\":5,\"tile_type\":\"w\"},{\"x\":2,\"y\":6,\"tile_type\":\"b\"},{\"x\":2,\"y\":7,\"tile_type\":\"w\"},{\"x\":3,\"y\":0,\"tile_type\":\"w\"},{\"x\":3,\"y\":1,\"tile_type\":\"b\"},{\"x\":3,\"y\":2,\"tile_type\":\"w\"},{\"x\":3,\"y\":3,\"tile_type\":\"b\"},{\"x\":3,\"y\":4,\"tile_type\":\"w\"},{\"x\":3,\"y\":5,\"tile_type\":\"b\"},{\"x\":3,\"y\":6,\"tile_type\":\"w\"},{\"x\":3,\"y\":7,\"tile_type\":\"b\"},{\"x\":4,\"y\":0,\"tile_type\":\"b\"},{\"x\":4,\"y\":1,\"tile_type\":\"w\"},{\"x\":4,\"y\":2,\"tile_type\":\"b\"},{\"x\":4,\"y\":3,\"tile_type\":\"w\"},{\"x\":4,\"y\":4,\"tile_type\":\"b\"},{\"x\":4,\"y\":5,\"tile_type\":\"w\"},{\"x\":4,\"y\":6,\"tile_type\":\"b\"},{\"x\":4,\"y\":7,\"tile_type\":\"w\"},{\"x\":5,\"y\":0,\"tile_type\":\"w\"},{\"x\":5,\"y\":1,\"tile_type\":\"b\"},{\"x\":5,\"y\":2,\"tile_type\":\"w\"},{\"x\":5,\"y\":3,\"tile_type\":\"b\"},{\"x\":5,\"y\":4,\"tile_type\":\"w\"},{\"x\":5,\"y\":5,\"tile_type\":\"b\"},{\"x\":5,\"y\":6,\"tile_type\":\"w\"},{\"x\":5,\"y\":7,\"tile_type\":\"b\"},{\"x\":6,\"y\":0,\"tile_type\":\"b\"},{\"x\":6,\"y\":1,\"tile_type\":\"w\"},{\"x\":6,\"y\":2,\"tile_type\":\"b\"},{\"x\":6,\"y\":3,\"tile_type\":\"w\"},{\"x\":6,\"y\":4,\"tile_type\":\"b\"},{\"x\":6,\"y\":5,\"tile_type\":\"w\"},{\"x\":6,\"y\":6,\"tile_type\":\"b\"},{\"x\":6,\"y\":7,\"tile_type\":\"w\"},{\"x\":7,\"y\":0,\"tile_type\":\"w\"},{\"x\":7,\"y\":1,\"tile_type\":\"b\"},{\"x\":7,\"y\":2,\"tile_type\":\"w\"},{\"x\":7,\"y\":3,\"tile_type\":\"b\"},{\"x\":7,\"y\":4,\"tile_type\":\"w\"},{\"x\":7,\"y\":5,\"tile_type\":\"b\"},{\"x\":7,\"y\":6,\"tile_type\":\"w\"},{\"x\":7,\"y\":7,\"tile_type\":\"b\"}],\"pieces\":[{\"owner\":0,\"x\":4,\"y\":0,\"piece_type\":\"k\"},{\"owner\":0,\"x\":3,\"y\":0,\"piece_type\":\"q\"},{\"owner\":0,\"x\":2,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":5,\"y\":0,\"piece_type\":\"b\"},{\"owner\":0,\"x\":1,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":6,\"y\":0,\"piece_type\":\"n\"},{\"owner\":0,\"x\":0,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":7,\"y\":0,\"piece_type\":\"r\"},{\"owner\":0,\"x\":0,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":1,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":2,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":3,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":4,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":5,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":6,\"y\":1,\"piece_type\":\"p\"},{\"owner\":0,\"x\":7,\"y\":1,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":7,\"piece_type\":\"k\"},{\"owner\":1,\"x\":3,\"y\":7,\"piece_type\":\"q\"},{\"owner\":1,\"x\":2,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":5,\"y\":7,\"piece_type\":\"b\"},{\"owner\":1,\"x\":1,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":6,\"y\":7,\"piece_type\":\"n\"},{\"owner\":1,\"x\":0,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":7,\"y\":7,\"piece_type\":\"r\"},{\"owner\":1,\"x\":0,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":1,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":2,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":3,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":4,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":5,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":6,\"y\":6,\"piece_type\":\"p\"},{\"owner\":1,\"x\":7,\"y\":6,\"piece_type\":\"p\"}],\"movement_patterns\":{}}}");
    let gameState = rawData['content'];

    let isPublic = true;
    let allowEdits = false;

    function onSaveChanges(e){
        gameState = e.detail;
        showBoardEditor = false;
    }


    function handleRoomRequest(e){
        $redirect('/chess/'+ e.detail.room_id);
    }


    function onCreateRoom(e){
        active = false;

        console.log("Creating room" + isPublic + gameState + allowEdits);
        createRoom(isPublic, gameState, allowEdits, function(v){
            $redirect('/chess/'+ v);
        });
    }


</script>
<style>
    #content {
        display: flex;
        align-items: center;
        flex-direction: column;
    }
    #createRoomRoomListWrapper {
        display: grid;
        justify-items: center;
        grid-template-columns: 1fr 1fr 1fr;
        grid-gap: 2em;
        width: 100%;
    }

    @media (max-width: 950px) {
        #createRoomRoomListWrapper {
            grid-template-columns: 1fr;
        }
    }
    #roomListWrapper {
        min-height: 50vh;
    }
    #createRoomWrapper {
        width:100%;
        max-width: 500px;
    }
</style>

<div id="content">
    <MediaQuery>
        <div slot="match" id="createRoomRoomListWrapper">
            <div class="container box" id="roomListWrapper">
                <Rl on:roomRequest={handleRoomRequest} roomList={$RoomList}/>
            </div>
            <div class="box container">
                <ProtochessInfo/>
            </div>
            <div>
                <div class="box container">
                    <PlayButtons on:requestPlayOnline={()=> active = true}/>
                </div>
            </div>
        </div>
        <div slot="fallback">
            <div class="box container">
                <PlayButtons on:requestPlayOnline={()=> active = true}/>
            </div>
            <Tabs position="is-centered" style="is-large">
                <Tab label="Info">
                    <div class="box container">
                        <ProtochessInfo/>
                    </div>
                </Tab>
                <Tab label="Rooms">
                    <div class="container box" id="roomListWrapper">
                        <Rl on:roomRequest={handleRoomRequest} roomList={$RoomList}/>
                    </div>
                </Tab>
            </Tabs>
        </div>
    </MediaQuery>
    <Modal bind:active={active}  onBody={false}> >
        <div style="display: flex; justify-content: center">
            <div class="box" id="createRoomWrapper">
                <CreateRoomDialog {gameState} bind:allowEdits={allowEdits} bind:isPublic={isPublic} on:createRoom={onCreateRoom} on:editBoard={()=> showBoardEditor = true}/>
            </div>
        </div>
    </Modal>

    {#if showBoardEditor}
        <FullWidthModal on:close={()=>showBoardEditor=false}>
            <div slot="header" style="text-align: center">
                <h1 class="title">Chess Editor</h1>
            </div>
            <ChessEditor on:saveChanges={onSaveChanges}  />
        </FullWidthModal>
    {/if}

</div>

