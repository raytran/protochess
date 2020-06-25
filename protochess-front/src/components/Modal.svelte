<script>
    import { createEventDispatcher, onDestroy, onMount } from 'svelte';
    let zIndex = 999999;

    const dispatch = createEventDispatcher();
    const close = () => dispatch('close');

    let modal;

    const handle_keydown = e => {
        if (e.key === 'Escape') {
            close();
            return;
        }

        if (e.key === 'Tab') {
            // trap focus
            const nodes = modal.querySelectorAll('*');
            const tabbable = Array.from(nodes).filter(n => n.tabIndex >= 0);

            let index = tabbable.indexOf(document.activeElement);
            if (index === -1 && e.shiftKey) index = 0;

            index += tabbable.length + (e.shiftKey ? -1 : 1);
            index %= tabbable.length;

            tabbable[index].focus();
            e.preventDefault();
        }
    };

    const previously_focused = typeof document !== 'undefined' && document.activeElement;

    if (previously_focused) {
        onDestroy(() => {
            previously_focused.focus();
        });
    }
</script>

<svelte:window on:keydown={handle_keydown}/>

<style>
    .mmodal-background {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0,0,0,0.3);
    }

    .mmodal-wrapper {
        display: flex;
        top:0;
        left:0;
        height: 100%;
        width: 100%;
        position: absolute;
        justify-content: center;
        align-items: center;
    }

    .mmodal {
        position: relative;
        width: 96%;
        height: 96%;
        overflow: scroll;
        transform: translate3d(0, 0, 0);
        padding: 1em;
        border-radius: 0.2em;
        background: white;
    }

</style>

<div style="z-index: {zIndex}" class="mmodal-background" on:click={close}></div>
<div style="z-index: {zIndex}" class="mmodal-wrapper">
    <div class="mmodal" role="dialog" aria-modal="true" bind:this={modal}>
        <slot name="header"></slot>
        <hr>
        <slot></slot>
        <hr>
    </div>
</div>
