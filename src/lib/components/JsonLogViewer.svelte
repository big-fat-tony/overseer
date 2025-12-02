<script>
    export let entries = []

    let processed = []
    let query = ""

    function enrich(list) {
        return list.map(e => ({
            ...e,
            clientTs: e.clientTs ?? Date.now()
        }))
    }

    $: processed = enrich(entries)

    function match(entry, text) {
        if (!text) return true
        const t = text.toLowerCase()
        return (
            entry.source?.toLowerCase().includes(t) ||
            String(entry.clientTs).includes(t) ||
            JSON.stringify(entry.payload).toLowerCase().includes(t)
        )
    }

    $: filtered = processed.filter(e => match(e, query))
</script>

<section class="log-view">
    <div class="controls">
        <input
                type="text"
                bind:value={query}
                placeholder="Search logs…"
                class="search"
        />

        <div class="count">
            {filtered.length} / {processed.length}
        </div>
    </div>

    <div class="log-container">
        {#if filtered.length === 0}
            <div class="empty">No matching entries.</div>
        {:else}
            {#each filtered as entry, i}
                <div class="log-line">
                    <div class="line-meta">
                        <span class="idx">#{i + 1}</span>
                        <span class="ts">{new Date(entry.clientTs).toLocaleTimeString()}</span>
                        <span class="src">{entry.source}</span>
                    </div>

                    <pre class="json">
{JSON.stringify(entry.payload, null, 2)}
                    </pre>
                </div>
            {/each}
        {/if}
    </div>
</section>

<style>
    .log-view {
        display: grid;
        grid-template-columns: 1fr;
        grid-template-rows: auto 1fr;
        grid-gap: 14px;
        height: 79vh;
    }

    .controls {
        display: flex;
        align-items: center;
        gap: 12px;
        background: var(--surface);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        border: 1px solid var(--border);
        padding: 12px 16px;
    }

    .search {
        flex: 1;
        padding: 10px 12px;
        font-size: 1rem;
        border-radius: var(--radius);
        border: 1px solid var(--border);
        background: var(--bg);
        color: var(--text);
        transition: border-color 0.2s;
    }

    .search:focus {
        border-color: var(--accent);
        outline: none;
    }

    .count {
        font-size: 0.9rem;
        font-weight: 600;
        color: var(--text);
        opacity: 0.7;
        min-width: 70px;
        text-align: right;
    }

    .log-container {
        overflow-y: auto;
        background: var(--surface);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        border: 1px solid var(--border);
        padding: 12px 16px;
    }

    .log-line {
        border-bottom: 1px solid var(--border);
        padding: 10px 0;
    }

    .log-line:last-child {
        border-bottom: none;
    }

    .line-meta {
        font-size: 0.75rem;
        color: var(--text);
        opacity: 0.6;
        display: flex;
        gap: 12px;
        margin-bottom: 6px;
        font-weight: 500;
    }

    .idx {
        color: var(--accent);
        font-weight: 600;
    }

    .json {
        margin: 0;
        font-family: monospace;
        font-size: 0.85rem;
        background: var(--bg);
        padding: 12px;
        border-radius: var(--radius);
        border: 1px solid var(--border);
        white-space: pre-wrap;
        overflow-x: auto;
    }

    .empty {
        color: var(--text);
        opacity: 0.6;
        font-style: italic;
        padding: 20px;
        text-align: center;
    }
</style>
