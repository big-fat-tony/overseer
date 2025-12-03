<script>
    import { update } from '$lib/stores/update.js'
    import { get } from 'svelte/store'

    let installing = false
    let progress = 0

    update.installing.subscribe(v => installing = v)
    update.progress.subscribe(v => progress = v)
</script>

{#if installing}
    <div class="overlay">
        <div class="card">
            <div class="title">Installing update…</div>
            <progress max="1" value={progress}></progress>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(0,0,0,0.6);
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .card {
        background: var(--surface);
        padding: 20px;
        border-radius: var(--radius);
        width: 300px;
        display: flex;
        flex-direction: column;
        gap: 14px;
        color: var(--text);
        border: 1px solid var(--border);
    }
    .title {
        font-weight: 600;
    }
</style>
