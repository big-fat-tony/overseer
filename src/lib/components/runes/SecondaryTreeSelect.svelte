<script>
    import { createEventDispatcher } from "svelte"
    import { runesData } from "$lib/runes/data.js"
    import { iconUrl } from "$lib/iconUrl.js"

    export let primaryTreeId = null
    export let secondaryTreeId = null

    const dispatch = createEventDispatcher()
    let open = false

    $: available = runesData.filter(t => t.id !== primaryTreeId)

    $: picked = secondaryTreeId
        ? runesData.find(t => t.id === secondaryTreeId)
        : null

    function toggle() {
        if (!primaryTreeId) return
        open = !open
    }

    function pick(tree) {
        dispatch("change", { secondaryTree: { treeId: tree.id } })
        open = false
    }
</script>

<div class="secondary-path">
    <button
            class="selector"
            type="button"
            on:click={toggle}
            disabled={!primaryTreeId}
    >
        {#if picked}
            <img class="icon" src={iconUrl(picked.icon)} alt={picked.name} />
            <div class="text">
                <div class="name">{picked.name}</div>
            </div>
        {:else}
            <div class="empty"></div>
            <div class="text">
                <div class="name">Select Secondary Path</div>
                <div class="hint">Choose a different Path</div>
            </div>
        {/if}
    </button>

    {#if open}
        <div class="choices-inline">
            {#each available as tree}
                <button
                        class="choice"
                        type="button"
                        data-label={tree.name}
                        on:click={() => pick(tree)}
                >
                    <img src={iconUrl(tree.icon)} alt={tree.name} />
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .secondary-path {
        position: relative;
        display: grid;
        gap: 0.4rem;
        color: var(--text);
    }

    .selector {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        gap: 0.8rem;
        padding: 0.6rem 0.9rem;
        width: 100%;
        height: 58px;
        border-radius: 12px;
        background: rgba(255,255,255,0.04);
        border: 1px solid rgba(255,255,255,0.08);
        cursor: pointer;
        position: relative;
        z-index: 2;
        color: var(--text);
    }

    .selector:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .text {
        color: var(--text);
    }

    .name {
        font-weight: bold;
    }

    .hint {
        color: var(--text-muted);
    }

    .choices-inline {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 58px;
        background: rgba(20,20,32,0.95);
        border-radius: 12px;
        display: flex;
        align-items: center;
        padding-left: 0.9rem;
        gap: 0.7rem;
        animation: fadeIn 120ms cubic-bezier(0.16,1,0.3,1) forwards;
        z-index: 2;
    }

    .choice {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.2rem;
        position: relative;
    }

    .choice img {
        width: 38px;
        height: 38px;
    }

    /* tooltip */
    .choice:hover::after {
        content: attr(data-label);
        position: absolute;
        left: 50%;
        bottom: -32px;
        transform: translateX(-50%);
        background: rgba(0,0,0,0.85);
        padding: 4px 8px;
        border-radius: 6px;
        font-size: 0.75rem;
        white-space: nowrap;
        pointer-events: none;
        opacity: 1;
    }

    .choice::after {
        opacity: 0;
        transition: opacity 0.12s ease;
        color: white;
    }

    .empty {
        width: 46px;
        height: 46px;
        border-radius: 999px;
        background: radial-gradient(circle at top, #26263a, #0a0b12);
    }

    @keyframes fadeIn {
        from { opacity: 0; transform: scale(0.92); }
        to   { opacity: 1; transform: scale(1); }
    }
</style>
