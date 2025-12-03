<script>
    import { update } from '$lib/stores/update.js'
    import { get } from 'svelte/store'

    let info = null
    update.info.subscribe(v => info = v)

    const install = () => update.install()
</script>

{#if info}
    <div class="banner">
        <div class="text">
            Update {info.latest} available (current: {info.current})
        </div>
        <button class="action" on:click={install}>Install</button>
    </div>
{/if}

<style>
    .banner {
        position: fixed;
        bottom: 20px;
        right: 20px;
        background: var(--surface);
        padding: 14px 18px;
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        display: flex;
        align-items: center;
        gap: 12px;
        border: 1px solid var(--border);
    }
    .text {
        color: var(--text);
        font-weight: 500;
    }
    .action {
        background: var(--accent);
        padding: 6px 12px;
        border-radius: 4px;
        color: white;
        border: none;
        cursor: pointer;
    }
</style>
