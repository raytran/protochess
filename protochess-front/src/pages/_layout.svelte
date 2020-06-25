<script>
    import { Icon } from 'svelma'
    import {isActive, url} from '@sveltech/routify'
    const links =
            [
                ['/', 'Home'],
                ['./faq', 'FAQ'],
                ['./about', 'About'],
            ]
    import { onMount,afterUpdate } from 'svelte'
    onMount(() => {
        const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger'), 0);
        // Check if there are any navbar burgers
        if ($navbarBurgers.length > 0) {
            // Add a click event on each of them
            $navbarBurgers.forEach( el => {
                el.addEventListener('click', () => {
                    // Get the target from the "data-target" attribute
                    const target = el.dataset.target;
                    const $target = document.getElementById(target);
                    // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
                    el.classList.toggle('is-active');
                    $target.classList.toggle('is-active');
                });
            });
        }
    });
    // in case you want to close it after user selects a different menu option
    afterUpdate(() => {
        const $navbarBurgers = Array.prototype.slice.call(document.querySelectorAll('.navbar-burger.is-active,.navbar-menu.is-active'), 0);
        if ($navbarBurgers.length > 0) {
            $navbarBurgers.forEach( el => {
                el.classList.toggle('is-active');
            });
        }
    });
</script>

<nav class="navbar is-info" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <a class="navbar-item" href={$url('/')}>
            <h2 class="title" style="font-weight: bold; color:white">Protochess</h2>
        </a>
        <a role="button" class="navbar-burger burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample">
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </a>
    </div>

    <div id="navbarBasicExample" class="navbar-menu">
        <div class="navbar-start">
            {#each links as [path, name]}
                <a class="navbar-item" href={$url(path)}>
                    {name}
                </a>
            {/each}
        </div>
    </div>
</nav>
<div style="min-height: 72vh; padding-top: 1em" id="content">
    <slot/>
</div>
<br>
<footer class="footer">
    <div class="content has-text-centered">

        <a href="https://github.com/raytran/protochess" data-icon="octicon-star" data-size="large" aria-label="Star raytran/protochess on GitHub">
            <Icon pack="fab" size="is-medium" icon="github-square" />
        </a>
        <p>
            <strong>Protochess</strong> by Raymond Tran. The source code is licensed
            <a href="https://www.gnu.org/licenses/gpl-3.0.en.html">GNU GPLv3</a>.
        </p>
    </div>
</footer>