<script>
    import {createEventDispatcher} from "svelte"
    import Teleport from "$lib/components/Teleport.svelte"
    import {iconUrl} from "$lib/iconUrl.js"

    export let rune = null
    export let selected = false
    export let showText = false
    export let tooltip = true
    export let size = 48

    const dispatch = createEventDispatcher()

    let hovering = false
    let x = 0
    let y = 0

    function click() {
        dispatch("select", {id: rune.id, rune})
    }

    function move(e) {
        x = e.clientX
        y = e.clientY
    }

    $: cssSize = size + "px"
</script>

<button
        type="button"
        class="rune"
        class:selected={selected}
        style={`--size:${cssSize}`}
        on:click={click}
        on:mouseenter={() => hovering = true}
        on:mouseleave={() => hovering = false}
        on:mousemove={move}
>
    <img class="icon" src={iconUrl(rune.icon)} alt={rune.name} draggable="false"/>

    {#if showText}
        <div class="label-block">
            <div class="title">{rune.name.toUpperCase()}</div>
            {#if rune.shortDesc}
                <div class="short">{@html rune.shortDesc}</div>
            {/if}
        </div>
    {/if}
</button>

{#if tooltip && rune.longDesc && hovering}
    <Teleport x={x} y={y}>
        <div class="tooltip">
            <div class="tooltip-title">{rune.name.toUpperCase()}</div>
            <div class="tooltip-body">{@html rune.longDesc}</div>
        </div>
    </Teleport>
{/if}

<style>
    .rune {
        position: relative;
        display: grid;
        grid-auto-flow: column;
        align-items: center;
        gap: 0.6rem;
        padding: 0;
        margin: 0;
        background: none;
        border: none;
        cursor: pointer;
        color: var(--text);
    }

    .icon {
        width: var(--size);
        height: var(--size);
        border-radius: 999px;
        object-fit: contain;
        padding: 4px;
        background: radial-gradient(circle at 40% 20%, #e9e4d8, #4d3a15 60%, #130d06);
    }

    .rune.selected .icon {
        box-shadow: 0 0 12px rgba(255, 213, 130, 0.75);
        outline: 2px solid #f0e6d2;
    }

    .label-block {
        display: grid;
        gap: 0.25rem;
    }

    .title {
        font-size: 0.85rem;
        font-weight: 700;
    }

    .short {
        font-size: 0.75rem;
        opacity: 0.9;
        overflow: hidden;
        text-overflow: ellipsis;
        line-height: 1rem;
    }

    .tooltip {
        position: fixed;
        background: radial-gradient(circle at top, #1b1d2d, #06070b 70%);
        border: 1px solid rgba(240, 230, 210, 0.7);
        padding: 0.75rem;
        border-radius: 8px;
        max-width: 320px;
        pointer-events: none;
        z-index: 999999;
        opacity: 0;
        animation: fade-in 90ms ease forwards;
    }

    @keyframes fade-in {
        from {
            opacity: 0;
            transform: scale(0.98);
        }
        to {
            opacity: 1;
            transform: scale(1);
        }
    }

    .tooltip-title {
        font-weight: 700;
        margin-bottom: 0.35rem;
    }

    .tooltip-body {
        opacity: 1;
        line-height: 1.2rem;
    }

    @keyframes fade-in {
        from {
            opacity: 0;
            transform: translate(-50%, 4px);
        }
        to {
            opacity: 1;
            transform: translate(-50%, 0);
        }
    }
</style>
