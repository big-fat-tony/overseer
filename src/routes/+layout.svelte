<script>
    import { initLogsListener } from '$lib/stores/logs.js'
    import { features } from '$lib/stores/features.js'
    import { update } from '$lib/stores/update.js'
    import '../app.css'
    import '../theme.css'
    import Breadcrumbs from '$lib/components/Breadcrumbs.svelte'
    import UpdateNotification from '$lib/components/UpdateNotification.svelte'
    import UpdateProgress from '$lib/components/UpdateProgress.svelte'

    initLogsListener()
    features.refresh()
    update.check()
</script>

<main>
    <UpdateNotification />
    <UpdateProgress />

    <div class="nav">
        <Breadcrumbs />
    </div>

    <div class="surface">
        <slot />
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
