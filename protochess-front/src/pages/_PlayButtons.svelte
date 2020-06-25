<script>
    import { url, redirect } from '@sveltech/routify';
    import {createEventDispatcher} from "svelte";
    const dispatch = createEventDispatcher();
</script>

<a class="button is-info is-large is-fullwidth" href={$url('/singleplayer')}>
    Play against the Computer
</a>
<br>
<button class="button is-primary is-large is-fullwidth" on:click={() => dispatch('requestPlayOnline')}>
    Play Online
</button>
