<script>
    import {initLogsListener} from "$lib/stores/logs.js"
    import {features} from "$lib/stores/features.js"
    import "../app.css"
    import "../theme.css"

    import Breadcrumbs from "$lib/components/Breadcrumbs.svelte"

    initLogsListener()
    features.refresh()
</script>

<main>
    <div class="nav">
        <Breadcrumbs/>
    </div>

    <div class="surface">
        <slot/>
    </div>
</main>

<style>
    main {
        display: grid;
        height: 100vh;
        grid-gap: 20px;
        grid-template-rows: auto 1fr;
    }
</style>
