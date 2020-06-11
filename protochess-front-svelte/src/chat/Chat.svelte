<script>
    let comments = [];
    import {sendChatMessage, ChatMessage} from '../WebsocketStore';
    import { beforeUpdate, afterUpdate } from 'svelte';
    import {PlayersList} from '../WebsocketStore';

    ChatMessage.subscribe(value => {
        if (value != null) {
            console.log(value);
            comments = [...comments, {from: value.from, content: value.content}];
        }
    });

    function handleKeyDown(event) {
        if (event.key === 'Enter') {
            const text = event.target.value;
            if (!text) return;
            console.log(text)
            event.target.value = '';
            sendChatMessage(text);
            comments = [...comments, {from: $PlayersList.you, content: text} ];
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
        height: 100%;
        max-width: 500px;
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
    <h1 style="text-align: center">Richard&Mortimer</h1>
    <div class="scrollable" bind:this={div}>
        {#each comments as comment}
            <article>
            <span>
                <b style="color: { comment.from === $PlayersList.you ? 'blue' : ''}">{
                comment.from === $PlayersList.you ? comment.from + " (You)" : comment.from
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