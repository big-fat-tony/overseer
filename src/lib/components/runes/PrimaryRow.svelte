<script>
    import { createEventDispatcher } from "svelte"
    import { runesData } from "$lib/runes/data.js"
    import Rune from "$lib/components/runes/Rune.svelte"

    export let primaryTreeId = null
    export let rowIndex = 0
    export let selectedRuneId = null

    const dispatch = createEventDispatcher()
    let open = false

    $: tree = runesData.find(t => t.id === primaryTreeId)
    $: slot = tree ? tree.slots[rowIndex] : null

    $: pickedRune = slot && selectedRuneId
        ? slot.runes.find(r => r.id === selectedRuneId)
        : null

    $: label = rowIndex === 0 ? "Select a Keystone" : "Select a Rune"

    function toggle() {
        if (!slot) return
        open = !open
    }

    function pick(rune) {
        dispatch("change", {
            primaryRow: { rowIndex, runeId: rune.id }
        })
        open = false
    }
</script>

<div class="primary-row">

    <div
            class="collapsed"
            on:click={toggle}
            style="display: {open ? 'none' : 'grid'}"
    >
        {#if pickedRune}
            <Rune
                    rune={pickedRune}
                    selected={true}
                    showText={true}
                    tooltip={true}
                    size={48}
            />
        {:else}
            <div class="empty-icon"></div>

            <div class="text">
                <div class="label">{label}</div>
            </div>
        {/if}
    </div>

    <div
            class="expanded"
            style="display: {open ? 'grid' : 'none'}"
    >
        {#each slot?.runes || [] as rune}
            <button
                    class="choice"
                    on:click={() => pick(rune)}
            >
                <Rune
                        rune={rune}
                        selected={selectedRuneId === rune.id}
                        showText={false}
                        tooltip={true}
                        size={48}
                />
            </button>
        {/each}
    </div>

</div>

<style>
    .primary-row {
        background: rgba(255,255,255,0.03);
        border: 1px solid rgba(255,255,255,0.08);
        border-radius: 12px;
        padding: 0.6rem 1rem;
        height: 58px;
        display: grid;
        place-items: center;
        cursor: pointer;
        overflow: visible; /* <-- tooltip fix */
    }

    .collapsed {
        width: 100%;
        height: 100%;
        display: grid;
        grid-template-columns: auto 1fr;
        align-items: center;
        gap: 0.8rem;
    }

    .text {
        display: grid;
        align-content: center;
    }

    .label {
        font-size: 0.9rem;
        opacity: 0.75;
    }

    .empty-icon {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        background: radial-gradient(circle at top, #2a2d3e, #0b0c12);
    }

    .expanded {
        width: 100%;
        height: 100%;
        display: grid;
        grid-auto-flow: column;
        grid-auto-columns: 1fr;
        gap: 0.6rem;
        align-items: center;
        animation: fade 120ms ease forwards;
    }

    .choice {
        background: none;
        border: none;
        padding: 0;
        width: 48px;
        height: 48px;
        display: grid;
        place-items: center;
        cursor: pointer;
        justify-self: center;
    }

    @keyframes fade {
        from { opacity: 0; transform: scale(0.96); }
        to { opacity: 1; transform: scale(1); }
    }
</style>
