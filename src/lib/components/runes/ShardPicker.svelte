<script>
    import { createEventDispatcher } from "svelte"
    import Rune from "$lib/components/runes/Rune.svelte"

    const dispatch = createEventDispatcher()

    export let value = { 0: null, 1: null, 2: null }

    const rows = [
        // OFFENSE
        [
            { id: 5008, name: "Adaptive Force", icon: "/runes/stat_shards/Adaptive Force.png" },
            { id: 5005, name: "Attack Speed",    icon: "/runes/stat_shards/Attack Speed.png" },
            { id: 5007, name: "Ability Haste",   icon: "/runes/stat_shards/Ability Haste.png" }
        ],

        // FLEX
        [
            { id: 5008, name: "Adaptive Force",  icon: "/runes/stat_shards/Adaptive Force.png" },
            { id: 5010, name: "Movement Speed",  icon: "/runes/stat_shards/Movement Speed.png" },
            { id: 5001, name: "Health Scaling",  icon: "/runes/stat_shards/Health Scaling.png" }
        ],

        // DEFENSE
        [
            { id: 5011, name: "Health",                   icon: "/runes/stat_shards/Health.png" },
            { id: 5013, name: "Tenacity and Slow Resist", icon: "/runes/stat_shards/Tenacity and Slow Resist.png" },
            { id: 5001, name: "Health Scaling",           icon: "/runes/stat_shards/Health Scaling.png" }
        ]
    ];

    function pick(row, id) {
        value = { ...value, [row]: id }
        dispatch("change", { shards: value })   // NAMESPACED
    }
</script>

<div class="shards">
    <div class="label">Bonus Stats</div>

    <div class="rows">
        {#each rows as row, rowIndex}
            <div class="row">
                {#each row as shard}
                    <Rune
                            rune={shard}
                            selected={value[rowIndex] === shard.id}
                            size={30}
                            showText={false}
                            on:select={() => pick(rowIndex, shard.id)}
                    />
                {/each}
            </div>
        {/each}
    </div>
</div>

<style>
    .shards {
        display: grid;
        gap: 0.6rem;
        color: var(--text);
    }
    .label {
        font-size: 0.75rem;
        text-transform: uppercase;
        opacity: 0.7;
    }
    .rows {
        display: grid;
        gap: 0.6rem;
    }
    .row {
        display: grid;
        grid-auto-flow: column;
        gap: 0.6rem;
    }
</style>
