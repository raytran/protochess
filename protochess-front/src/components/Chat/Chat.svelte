<script>
    export let comments = [];
    const dispatch = createEventDispatcher();
    import {beforeUpdate, afterUpdate, createEventDispatcher} from 'svelte';

    export let name = "anon";


    function handleKeyDown(event) {
        if (event.key === 'Enter') {
            const text = event.target.value;
            if (!text) return;
            console.log(text)
            event.target.value = '';
            dispatch('chatRequest', text);
        }
    }

    let div;
    let autoscroll;
    beforeUpdate(() => {
        autoscroll = div && (div.offsetHeight + div.scrollTop) > (div.scrollHeight - 20);
    });

    afterUpdate(() => {
        if (autoscroll) div.scrollTo(0, div.scrollHeight);
    });

</script>
<style>
    .scrollable {
        flex: 1 1 auto;
        border-top: 1px solid #eee;
        margin: 0 0 0.5em 0;
        padding: 0.5em;
        overflow-y: auto;
    }
    .chat {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
    span {
        padding: 0.2em;
        display: inline-block;
    }
    input {
        margin: 0;
    }
</style>

<div class="chat">
    <h1 style="text-align: center">Chat</h1>
    <div class="scrollable" bind:this={div}>
        {#each comments as comment}
            <article>
            <span>
                <b style="color: { comment.from === name ? 'blue' : ''}">{
                comment.from === name ? comment.from + " (You)" : comment.from
                }</b>
                <br>
                &nbsp;&nbsp;&nbsp;&nbsp;
                {comment.content}
            </span>
            </article>
        {/each}
    </div>
    <input on:keydown={handleKeyDown}/>
</div>