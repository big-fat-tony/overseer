<script>
    import { goto } from "$app/navigation"
    import {
        saveRunePage,
        deleteRunePage
    } from "$lib/commands.js"

    import PrimaryTreeSelect from "$lib/components/runes/PrimaryTreeSelect.svelte"
    import SecondaryTreeSelect from "$lib/components/runes/SecondaryTreeSelect.svelte"
    import PrimaryRow from "$lib/components/runes/PrimaryRow.svelte"
    import SecondarySlot from "$lib/components/runes/SecondarySlot.svelte"
    import ShardPicker from "$lib/components/runes/ShardPicker.svelte"
    import ChampionPicker from "$lib/components/ChampionPicker.svelte"
    import RolePicker from "$lib/components/RolePicker.svelte"

    export let mode = "new"
    export let existingPage = null

    let id = null
    let buildName = ""
    let selectedChampions = []
    let role = null

    let primaryTreeId = null
    let secondaryTreeId = null

    let primaryPicks = { 0: null, 1: null, 2: null, 3: null }
    let secondaryPicks = { 0: null, 1: null }
    let shards = { 0: null, 1: null, 2: null }

    $: canSave =
        primaryTreeId &&
        secondaryTreeId &&
        buildName.trim().length > 0

    if (mode === "edit" && existingPage) {
        id = existingPage.id
        buildName = existingPage.name
        selectedChampions = existingPage.champions ?? []
        role = existingPage.role ?? null
        primaryTreeId = existingPage.primary_tree_id ?? null
        secondaryTreeId = existingPage.secondary_tree_id ?? null
        primaryPicks = existingPage.primary_picks ?? primaryPicks
        secondaryPicks = existingPage.secondary_picks ?? secondaryPicks
        shards = existingPage.shards ?? shards
    }

    async function saveNew() {
        const newId = crypto.randomUUID()

        const page = {
            id: newId,
            name: buildName,
            champions: selectedChampions,
            role,
            primary_tree_id: primaryTreeId,
            primary_picks: primaryPicks,
            secondary_tree_id: secondaryTreeId,
            secondary_picks: secondaryPicks,
            shards
        }

        await saveRunePage(page)
        goto("/features/RunePicker")
    }

    async function updateExisting() {
        const page = {
            id,
            name: buildName,
            champions: selectedChampions,
            role,
            primary_tree_id: primaryTreeId,
            primary_picks: primaryPicks,
            secondary_tree_id: secondaryTreeId,
            secondary_picks: secondaryPicks,
            shards
        }

        await saveRunePage(page)
        goto("/features/RunePicker")
    }

    async function deleteExisting() {
        await deleteRunePage(id)
        goto("/features/RunePicker")
    }

    const saveAction = mode === "new" ? saveNew : updateExisting

    function onPrimaryTreeChange(e) {
        const { treeId } = e.detail.primaryTree
        primaryTreeId = treeId
        primaryPicks = { 0: null, 1: null, 2: null, 3: null }
        secondaryTreeId = null
        secondaryPicks = { 0: null, 1: null }
    }

    function onSecondaryTreeChange(e) {
        const { treeId } = e.detail.secondaryTree
        secondaryTreeId = treeId
        secondaryPicks = { 0: null, 1: null }
    }

    function onPrimaryRowChange(e) {
        const { rowIndex, runeId } = e.detail.primaryRow
        primaryPicks = { ...primaryPicks, [rowIndex]: runeId }
    }

    function onSecondarySlotChange(e) {
        const { slotIndex, runeId } = e.detail.secondarySlot
        secondaryPicks = { ...secondaryPicks, [slotIndex]: runeId }
    }

    function onShardsChange(e) {
        shards = e.detail.shards
    }
</script>

<main class="rune-page">

    <section class="top-select">
        <PrimaryTreeSelect
                primaryTreeId={primaryTreeId}
                on:change={onPrimaryTreeChange}
        />

        <SecondaryTreeSelect
                primaryTreeId={primaryTreeId}
                secondaryTreeId={secondaryTreeId}
                on:change={onSecondaryTreeChange}
        />
    </section>

    <section class="grid">
        <div class="primary0">
            <PrimaryRow
                    primaryTreeId={primaryTreeId}
                    rowIndex={0}
                    selectedRuneId={primaryPicks[0]}
                    on:change={onPrimaryRowChange}
            />
        </div>

        <div class="secondary0">
            <SecondarySlot
                    secondaryTreeId={secondaryTreeId}
                    pickedRuneId={secondaryPicks[0]}
                    otherPickedRuneId={secondaryPicks[1]}
                    slotIndex={0}
                    on:change={onSecondarySlotChange}
            />
        </div>

        <div class="primary1">
            <PrimaryRow
                    primaryTreeId={primaryTreeId}
                    rowIndex={1}
                    selectedRuneId={primaryPicks[1]}
                    on:change={onPrimaryRowChange}
            />
        </div>

        <div class="secondary1">
            <SecondarySlot
                    secondaryTreeId={secondaryTreeId}
                    pickedRuneId={secondaryPicks[1]}
                    otherPickedRuneId={secondaryPicks[0]}
                    slotIndex={1}
                    on:change={onSecondarySlotChange}
            />
        </div>

        <div class="primary2">
            <PrimaryRow
                    primaryTreeId={primaryTreeId}
                    rowIndex={2}
                    selectedRuneId={primaryPicks[2]}
                    on:change={onPrimaryRowChange}
            />
        </div>

        <div class="shards">
            <ShardPicker
                    value={shards}
                    on:change={onShardsChange}
            />
        </div>

        <div class="primary3">
            <PrimaryRow
                    primaryTreeId={primaryTreeId}
                    rowIndex={3}
                    selectedRuneId={primaryPicks[3]}
                    on:change={onPrimaryRowChange}
            />
        </div>
    </section>

    <section class="save-panel">

        <h2>{mode === "new" ? "Save Rune Page" : "Edit Rune Page"}</h2>

        <input
                class="build-name"
                type="text"
                placeholder="Rune page name"
                bind:value={buildName}
        />

        <ChampionPicker bind:selectedChampions={selectedChampions} />

        <RolePicker bind:role={role} />

        <button
                class="save-button"
                on:click={saveAction}
                disabled={!canSave}
        >
            {mode === "new" ? "Save" : "Save Changes"}
        </button>

        {#if mode === "edit"}
            <button class="delete-button" on:click={deleteExisting}>
                Delete
            </button>
        {/if}

    </section>

</main>

<style>
    .rune-page {
        min-height: 100vh;
        padding: 2rem;
        display: grid;
        gap: 2rem;
    }

    .top-select {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
    }

    .grid {
        display: grid;
        grid-template-columns: 1.3fr 1fr;
        grid-template-rows: auto auto auto auto;
        grid-template-areas:
            "primary0 secondary0"
            "primary1 secondary1"
            "primary2 shards"
            "primary3 shards";
        gap: 1.2rem 2.2rem;
    }

    .primary0 { grid-area: primary0; }
    .primary1 { grid-area: primary1; }
    .primary2 { grid-area: primary2; }
    .primary3 { grid-area: primary3; }
    .secondary0 { grid-area: secondary0; }
    .secondary1 { grid-area: secondary1; }
    .shards { grid-area: shards; }

    .save-panel {
        display: grid;
        gap: 1.2rem;
        padding: 1.4rem;
        background: rgba(255,255,255,0.03);
        border: 1px solid rgba(255,255,255,0.08);
        border-radius: 12px;
    }

    .build-name {
        padding: 12px 14px;
        border-radius: var(--radius);
        border: 1px solid var(--border);
        background: var(--surface);
        color: var(--text);
    }

    .save-button {
        padding: 12px;
        border-radius: var(--radius);
        border: none;
        background: var(--accent);
        color: var(--surface);
        cursor: pointer;
        font-weight: bold;
    }

    .save-button:disabled {
        opacity: 0.4;
        cursor: default;
    }

    .delete-button {
        padding: 12px;
        border-radius: var(--radius);
        border: 1px solid rgba(255,0,0,0.4);
        background: rgba(255,0,0,0.1);
        color: #ff6b6b;
        cursor: pointer;
        font-weight: bold;
    }

    .delete-button:hover {
        background: rgba(255,0,0,0.2);
    }
</style>
