<script>
    import { createEventDispatcher } from "svelte";

    export let name;
    export let index;

    const dispatch = createEventDispatcher();

    const remove = () => {
        dispatch("remove", name);
    };
</script>

<div class="chip">
    <div class="index">{index + 1}</div>
    <div class="name">{name}</div>
    <button type="button" on:click={remove}>✕</button>
</div>

<style>
    .chip {
        display: flex;
        align-items: center;

        gap: 10px;
        padding: 8px 12px;
        background: var(--surface-translucent);
        border: 1px solid var(--border);
        border-radius: 12px;
        font-size: .95rem;
        cursor: grab;
        transition: background .12s ease, transform .12s ease, box-shadow .12s ease;
    }

    .chip:active {
        cursor: grabbing;
        background: var(--surface);
        box-shadow: 0 4px 12px rgba(0,0,0,0.05);
        transform: scale(1.01);
    }

    .index {
        width: 22px;
        height: 22px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: 700;
        border-radius: 999px;
        background: var(--accent);
        color: white;
        font-size: .75rem;
        flex-shrink: 0;
    }

    .name {
        flex: 1;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    button {
        background: none;
        border: none;
        font-size: 1rem;
        cursor: pointer;
        opacity: .7;
        padding: 0 2px;
        flex-shrink: 0;
        color: var(--text)
    }

    button:hover {
        opacity: 1;
    }
</style>
