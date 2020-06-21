<script>
    import {createEventDispatcher} from "svelte";
    export let roomList;
    const dispatch = createEventDispatcher();
</script>

<style>
    #wrapper {
        padding-top: 0.3em;
        box-shadow: 0 14px 28px rgba(0,0,0,0.25), 0 10px 10px rgba(0,0,0,0.22);
        width: 100%;
        text-align:center;
    }
    #content {
        padding: 1em;
        border-top: 1px solid #eee;
    }
    li { cursor: pointer; }
    li:hover {
        background-color: #0bf5cc;
    }
</style>


<div id="wrapper">
    <h3>Public Rooms</h3>
    <div id="content">
        <ul>
            {#each roomList as roomInfo}
                <li on:click={dispatch("roomRequest", roomInfo)}>
                    <div>
                        {roomInfo.room_id}
                    </div>
                    <div>
                        {roomInfo.num_clients} Players
                    </div>
                </li>
            {/each}
        </ul>
    </div>
</div>
