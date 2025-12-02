<script>
    export let role = null

    const roles = [
        "top",
        "jungle",
        "mid",
        "bottom",
        "support"
    ]

    function choose(r) {
        role = r
    }
</script>

<div class="role-picker">
    {#each roles as r}
        <button
                type="button"
                class:selected={role === r}
                on:click={() => choose(r)}
        >
            {r}
        </button>
    {/each}
</div>

<style>
    .role-picker {
        display: flex;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .role-picker button {
        padding: 8px 14px;
        border-radius: 8px;
        background: var(--surface);
        border: 1px solid var(--border);
        cursor: pointer;
        color: var(--text);
        font-size: 0.9rem;
        transition: background 0.15s ease;
    }

    .role-picker button.selected {
        background: var(--accent);
        color: var(--surface);
        border-color: var(--accent);
        font-weight: bold;
    }

    .role-picker button:hover:not(.selected) {
        background: rgba(255,255,255,0.08);
    }
</style>
