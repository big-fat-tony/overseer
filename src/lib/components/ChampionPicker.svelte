<script>
    import { onMount } from "svelte"
    import { listChampions } from "$lib/commands.js"

    export let selectedChampions = []

    let champions = []
    let query = ""
    let filtered = []
    let open = false

    onMount(async () => {
        champions = await listChampions()
        filtered = champions
    })

    const search = () => {
        filtered = champions.filter(([n]) =>
            n.toLowerCase().includes(query.toLowerCase())
        )
        open = query.length > 0
    }

    const pick = (champ) => {
        const name = champ[0]
        if (!selectedChampions.includes(name)) {
            selectedChampions = [...selectedChampions, name]
        }
        query = ""
        open = false
        filtered = champions
    }

    const remove = (name) => {
        selectedChampions = selectedChampions.filter(c => c !== name)
    }
</script>

<div class="champ-picker">

    <div class="chips">
        {#each selectedChampions as champ}
            <span class="chip">
                {champ}
                <button class="remove" on:click={() => remove(champ)}>✕</button>
            </span>
        {/each}
    </div>

    <div class="autocomplete">
        <input
                type="text"
                bind:value={query}
                on:input={search}
                on:focus={() => open = true}
                placeholder="Assign to champions..."
        />

        {#if open}
            <ul class="dropdown">
                {#each filtered as champ}
                    <li>
                        <button
                                type="button"
                                class="option"
                                on:click={() => pick(champ)}
                        >
                            {champ[0]}
                        </button>
                    </li>
                {/each}
            </ul>
        {/if}
    </div>

</div>

<style>
    .champ-picker { display: grid; gap: 0.8rem; }

    .chips {
        display: flex;
        gap: 0.4rem;
        flex-wrap: wrap;
    }

    .chip {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        padding: 6px 10px;
        border-radius: 999px;
        background: var(--surface);
        border: 1px solid var(--border);
    }

    .remove {
        background: none;
        border: none;
        cursor: pointer;
        color: var(--text-muted);
    }

    .autocomplete {
        position: relative;
    }

    .autocomplete input {
        caret-color: var(--text);
        color: var(--text);
        width: 100%;
        padding: 12px 14px;
        border-radius: var(--radius);
        border: 1px solid var(--border);
        background: var(--surface);
    }

    .dropdown {
        position: absolute;
        top: calc(100%);
        left: 0;
        right: 0;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 6px 0;
        max-height: 220px;
        overflow-y: auto;
        box-shadow: var(--shadow);
        z-index: 30;
    }

    .option {
        width: 100%;
        padding: 10px 14px;
        text-align: left;
        background: none;
        border: none;
        cursor: pointer;
        color: var(--text);
    }

    .option:hover {
        background: var(--accent);
        color: var(--surface);
    }
</style>
