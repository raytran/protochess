<script>
    import { Icon } from 'svelma'
</script>

<div class="container">
    <div class="box">
        <h1 class="title">Protochess by Raymond Tran</h1>
        <h3 class="subtitle">
            <a href="https://github.com/raytran/protochess" data-icon="octicon-star" data-size="large" aria-label="Star raytran/protochess on GitHub">
                It's on Github
                <Icon pack="fab" size="is-medium" icon="github" />
            </a>
        </h3>
        The frontend is written in <a href="https://svelte.dev/">Svelte</a> with routing from <a href="https://routify.dev/">Routify</a> and styling with the <a href="https://bulma.io/">Bulma</a> CSS framework.
        <br>
        All the chess logic is written in <a href="https://www.rust-lang.org/">Rust</a>, and compiled to <a href="https://www.rust-lang.org/what/wasm">WebAssembly</a> to run singleplayer. The multiplayer websocket server uses <a href="https://github.com/seanmonstar/warp">Warp</a>.
    </div>
</div>

