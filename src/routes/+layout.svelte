<script>
    import { initLogsListener } from '$lib/stores/logs.js'
    import { features } from '$lib/stores/features.js'
    import { update } from '$lib/stores/update.js'
    import { version } from '$lib/stores/version.js'
    import '../app.css'
    import '../theme.css'
    import Breadcrumbs from '$lib/components/Breadcrumbs.svelte'
    import UpdateNotification from '$lib/components/UpdateNotification.svelte'
    import UpdateProgress from '$lib/components/UpdateProgress.svelte'

    let v = null
    version.value.subscribe(x => v = x)

    initLogsListener()
    features.refresh()
    update.check()
    version.load()
</script>

<main>
    <UpdateNotification />
    <UpdateProgress />

    <div class="topbar">
        <div class="nav">
            <Breadcrumbs />
        </div>
        <div class="ver">{#if v}v{v}{/if}</div>
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

    .topbar {
        display: grid;
        grid-template-columns: 1fr auto;
        align-items: center;
        padding: 8px 14px;
        background: var(--surface);
        border-bottom: 1px solid var(--border);
    }

    .nav {
        display: flex;
        align-items: center;
    }

    .ver {
        opacity: 0.7;
        font-size: 0.9rem;
    }
</style>
