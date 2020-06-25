<script>
    import {createEventDispatcher} from "svelte";
    export let roomList;
    const dispatch = createEventDispatcher();
</script>

<style>
    #wrapper {
        width: 100%;
        text-align:center;
    }
    #content {
        padding: 1em;
        border-top: 1px solid #eee;
    }
    table > tr:hover {
        background-color: #0bf5cc;
        cursor:pointer;
    }
</style>


<div id="wrapper">
    <h1 class="title">Public Rooms</h1>
    <div id="content" class="table-container">
        <table class="table is-fullwidth">
            <thead>
            <tr>
                <th>Room Id</th>
                <th>Edits on?</th>
                <th># Clients</th>
            </tr>
            </thead>
            {#each roomList.sort((a, b) => a.num_clients > b.num_clients) as roomInfo}
                <tr on:click={dispatch("roomRequest", roomInfo)} >
                    <td>{roomInfo.room_id}</td>
                    <td>{roomInfo.editable ? 'Yes' : 'No'} </td>
                    <td>{roomInfo.num_clients}</td>
                </tr>
            {/each}
        </table>
    </div>
</div>
