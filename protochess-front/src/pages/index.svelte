<script>
    import {RoomList, createRoom} from '../WebsocketStore';
    import { redirect } from '@sveltech/routify'

    import Rl from "../components/RoomList/RoomList.svelte";
    import CreateRoomDialog from "../components/CreateRoomDialog/CreateRoomDialog.svelte";
    import ChessEditor from "../components/ChessEditor/ChessEditor.svelte";
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
    #createRoomWrapper{
        width: 100%;
        max-width: 400px;
    }
    #createRoomRoomListWrapper {
        display: grid;
        justify-items: center;
        grid-template-columns: 1fr 1fr;
        width: 100%;
    }

    @media (max-width: 850px) {
        #createRoomRoomListWrapper {
            grid-template-columns: 1fr;
        }
    }
    #roomListWrapper {
        max-width: 400px;
        width: 100%;
    }
</style>

<div id="content">
    <h3>Chess with custom pieces and boards</h3>
    <div>
        Design your pieces & play against your friends.
    </div>
    {#if showBoardEditor}
        <ChessEditor on:saveChanges={onSaveChanges}  />
    {/if}
    <div id="createRoomRoomListWrapper">
        <div id="createRoomWrapper">
            <CreateRoomDialog {gameState} bind:allowEdits={allowEdits} bind:isPublic={isPublic} on:createRoom={onCreateRoom} on:editBoard={()=> showBoardEditor = true}/>
        </div>

        <div id="roomListWrapper">
            <Rl on:roomRequest={handleRoomRequest} roomList={$RoomList}/>
        </div>
    </div>
</div>
