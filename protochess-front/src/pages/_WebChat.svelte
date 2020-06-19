
<script>
    import Chat from "../components/Chat/Chat.svelte";
    import {PlayersList, ChatMessage, sendChatMessage} from "../WebsocketStore";
    let comments = [];

    ChatMessage.subscribe(value => {
        if (value != null) {
            console.log(value);
            comments = [...comments, {from: value.from, content: value.content}];
        }
    });
    function onChatRequest(e) {
        sendChatMessage(e.detail)
        comments = [...comments, {from: $PlayersList.you, content: e.detail} ];
    }

</script>

<Chat on:chatRequest={onChatRequest} name={$PlayersList.you} {comments}/>
