<script>
    import { createEventDispatcher } from "svelte";
    import { dndzone } from "svelte-dnd-action";
    import PreferenceChip from "./PreferenceChip.svelte";

    export let type;
    export let data = [];
    export let add;   /* function passed from parent */

    const dispatch = createEventDispatcher();

    let items = [];
    $: items = data.map((name) => ({ id: name, name }));

    const remove = (name) => {
        dispatch("update", data.filter((d) => d !== name));
    };

    const handleFinalize = (event) => {
        const next = event.detail.items.map((i) => i.id);
        dispatch("update", next);
    };
</script>

<div class="section">
    <div class="label">{type}s</div>

    <div class="input-wrap">
        <slot name="input" />
    </div>

    <div
            class="list"
            use:dndzone={{
            items,
            flipDurationMs: 150,
            dragDisabled: false,
            dropFromOthersDisabled: true
        }}
            on:finalize={handleFinalize}
    >
        {#each items as item, index (item.id)}
            <PreferenceChip
                    name={item.name}
                    index={index}
                    on:remove={() => remove(item.name)}
            />
        {/each}
    </div>
</div>

<style>
    .section {
        display: grid;
        gap: 12px;
        min-width: 0;          /* 🔥 allows content to shrink inside grid */
    }

    .label {
        font-size: .8rem;
        letter-spacing: .05em;
        text-transform: uppercase;
        opacity: .65;
        min-width: 0;          /* avoids text expanding container */
    }

    .input-wrap {
        display: block;
        min-width: 0;
    }

    .list {
        display: flex;
        flex-direction: column;
        gap: 8px;
        min-width: 0;          /* 🔥 prevents overflow from wide chip labels */
        overflow-x: hidden;    /* safe fallback */
    }
</style>
