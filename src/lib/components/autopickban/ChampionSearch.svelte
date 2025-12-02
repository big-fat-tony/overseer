<script>
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import { listChampions } from "$lib/commands.js";

    const dispatch = createEventDispatcher();

    let champions = [];
    let query = "";
    let filtered = [];
    let open = false;

    onMount(async () => {
        champions = await listChampions();
        filtered = champions;
    });

    const search = () => {
        filtered = champions.filter(([n]) =>
            n.toLowerCase().includes(query.toLowerCase())
        );
        open = query.length > 0;
    };

    const pick = (champ) => {
        query = champ[0];
        open = false;
        dispatch("select", champ);
    };
</script>

<div class="autocomplete">
    <input
            type="text"
            bind:value={query}
            on:input={search}
            on:focus={() => (open = true)}
    />

    {#if open}
        <ul class="dropdown">
            {#each filtered as champ}
                <li>
                    <button type="button" class="option" on:click={() => pick(champ)}>
                        {champ[0]}
                    </button>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style>
    .autocomplete {
        position: relative;
    }

    .autocomplete input {
        caret-color: var(--text);
        color: var(--text);
        padding: 12px 14px;
        border-radius: var(--radius);
        border: 1px solid var(--border);
        background: var(--surface);
        transition: box-shadow .15s ease;
    }

    .autocomplete input:focus {
        outline: none;
        box-shadow: 0 0 0 3px rgba(58, 109, 240, 0.25);
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
        animation: fadeSlide .18s ease;
        z-index: 30;
    }

    .option {
        width: 100%;
        padding: 10px 14px;
        text-align: left;
        background: none;
        border: none;
        font-size: .95rem;
        cursor: pointer;
        color: var(--text);
    }

    .option:hover {
        background: var(--accent);
        color: var(--surface);
    }

    @keyframes fadeSlide {
        from { opacity: 0; transform: translateY(4px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>
