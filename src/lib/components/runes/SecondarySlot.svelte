<script>
    import {createEventDispatcher} from "svelte"
    import {runesData} from "$lib/runes/data.js"
    import Rune from "$lib/components/runes/Rune.svelte"

    export let secondaryTreeId = null
    export let pickedRuneId = null
    export let otherPickedRuneId = null
    export let slotIndex = 0      // 0 or 1

    const dispatch = createEventDispatcher()
    let open = false

    // -------------------------------
    // Resolve tree + runes (excluding keystone row)
    // -------------------------------
    $: tree = runesData.find(t => t.id === secondaryTreeId)

    // Secondary uses rows 1, 2, 3 → flatten into 9 runes
    $: availableSlots = tree ? tree.slots.slice(1) : []
    $: allRunes = availableSlots.flatMap(s => s.runes)

    // Selected rune object (null-safe)
    $: pickedRune = pickedRuneId
        ? allRunes.find(r => r.id === pickedRuneId)
        : null

    // Label shown when empty
    $: label = "Select a Rune"

    // -------------------------------
    // UI toggle
    // -------------------------------
    function toggle() {
        if (!tree) return
        open = !open
    }

    // -------------------------------
    // Handle picking
    // -------------------------------
    function pick(rune) {
        dispatch("change", {
            secondarySlot: {
                slotIndex,
                runeId: rune.id
            }
        })
        open = false
    }
</script>

<div class="secondary-slot">
    <!-- COLLAPSED VIEW -->
    {#if !open}
        <button
                class="collapsed"
                on:click={toggle}
                disabled={!tree}
        >
            {#if pickedRune}
                <Rune
                        rune={pickedRune}
                        selected={true}
                        showText={true}
                        size={38}
                        tooltip={true}
                />
            {:else}
                <div class="empty-icon"></div>
                <div class="text">
                    <div class="label">{label}</div>
                </div>
            {/if}
        </button>
    {/if}

    <!-- EXPANDED GRID -->
    {#if open}
        <div class="expanded">
            {#each allRunes as rune}
                <button
                        class="choice"
                        on:click={() => pick(rune)}
                >
                    <Rune
                            rune={rune}
                            selected={pickedRuneId === rune.id}
                            showText={false}
                            tooltip={true}
                            size={38}
                    />
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .secondary-slot {
        display: grid;
        color: var(--text);
    }

    .collapsed {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 0.8rem;
        padding: 0.6rem 1rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 12px;
        cursor: pointer;
        text-align: left;
    }

    .collapsed:disabled {
        cursor: default;
        opacity: 0.4;
    }

    .empty-icon {
        width: 38px;
        height: 38px;
        border-radius: 50%;
        background: radial-gradient(circle at top, #2a2d3e, #0b0c12);
    }

    .text {
        display: grid;
        align-content: center;
    }

    .label {
        font-size: 0.9rem;
        opacity: 0.8;
        color: var(--text);
    }

    .expanded {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
        gap: 1rem;
        padding: 1rem;
        background: rgba(20, 20, 32, 0.95);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        animation: open 120ms cubic-bezier(0.16, 1, 0.3, 1) forwards;
    }

    .choice {
        background: transparent;
        border: none;
        cursor: pointer;
        text-align: left;
    }

    @keyframes open {
        from {
            opacity: 0;
            transform: scale(0.92);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }
</style>
