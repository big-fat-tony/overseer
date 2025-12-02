<script>
    import { onMount } from "svelte";
    import { listRunePages } from "$lib/commands.js";
    import { goto } from "$app/navigation";
    import { runesData } from "$lib/runes/data.js";
    import { iconUrl } from "$lib/iconUrl.js";

    let pages = [];

    onMount(async () => {
        pages = await listRunePages();
    });

    function openPage(id) {
        goto(`/features/RunePicker/${id}`);
    }

    function createNew() {
        goto("/features/RunePicker/new");
    }

    function findTree(id) {
        return runesData.find(t => t.id === id);
    }
</script>

<main class="rune-list">
    <section class="header">
        <h1>Rune Pages</h1>
        <button class="new-button" on:click={createNew}>
            New Rune Page
        </button>
    </section>

    <section class="items">
        {#each pages as page}
            {@const primaryTree = findTree(page.primary_tree_id)}
            {@const secondaryTree = findTree(page.secondary_tree_id)}

            <article class="item" on:click={() => openPage(page.id)}>

                <div class="icons">
                    {#if primaryTree}
                        <img
                                class="tree-icon primary"
                                src={iconUrl(primaryTree.icon)}
                                alt="primary tree"
                        />
                    {/if}

                    {#if secondaryTree}
                        <img
                                class="tree-icon secondary"
                                src={iconUrl(secondaryTree.icon)}
                                alt="secondary tree"
                        />
                    {/if}
                </div>

                <div class="info">
                    <h3>{page.name}</h3>

                    {#if page.champions?.length}
                        <p>{page.champions.join(", ")}</p>
                    {/if}

                    {#if page.role}
                        <p>{page.role}</p>
                    {/if}
                </div>
            </article>
        {/each}
    </section>
</main>

<style>
    .rune-list {
        padding: 2rem;
        display: grid;
        gap: 1.5rem;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .items {
        display: grid;
        gap: 1rem;
    }

    .item {
        padding: 1rem;
        border-radius: 12px;
        border: 1px solid rgba(255,255,255,0.08);
        background: rgba(255,255,255,0.03);
        cursor: pointer;
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    .icons {
        display: flex;
        gap: 0.4rem;
        align-items: center;
    }

    .tree-icon {
        width: 36px;
        height: 36px;
        border-radius: 999px;
        background: radial-gradient(circle at 40% 20%, #e9e4d8, #4d3a15 60%, #130d06);
        padding: 4px;
    }

    .primary {
        box-shadow: 0 0 10px rgba(255, 215, 130, 0.6);
    }

    .secondary {
        opacity: 0.75;
    }

    .info {
        display: grid;
        gap: 0.1rem;
    }

    h3 {
        margin: 0;
    }

    .new-button {
        padding: 0.6rem 1.2rem;
        border-radius: 999px;
        border: none;
        background: var(--accent);
        color: var(--surface);
        font-weight: 600;
        cursor: pointer;
    }
</style>
