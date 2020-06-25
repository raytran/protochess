<script>
    import { url, redirect } from '@sveltech/routify'
    import {createEventDispatcher} from "svelte";
    const dispatch = createEventDispatcher();
</script>

<button class="button is-info is-large is-fullwidth" on:click={() => $redirect('../singleplayer')}>
    Play against the Computer
</button>
<br>
<button class="button is-primary is-large is-fullwidth" on:click={() => dispatch('requestPlayOnline')}>
    Play Online
</button>
