<script>
    import { onMount, onDestroy } from "svelte"

    export let x = 0
    export let y = 0
    export let offset = 12

    let el

    onMount(() => {
        document.body.appendChild(el)
    })

    onDestroy(() => {
        if (el && el.parentNode) el.parentNode.removeChild(el)
    })

    $: {
        const vw = window.innerWidth
        const vh = window.innerHeight

        const leftSide = x < vw / 2
        const topSide = y < vh / 2

        let px = x
        let py = y

        if (leftSide) px = x + offset
        else px = x - offset

        if (topSide) py = y + offset
        else py = y - offset

        el?.style.setProperty("left", px + "px")
        el?.style.setProperty("top", py + "px")
    }
</script>

<div bind:this={el} class="teleport">
    <slot />
</div>

<style>
    .teleport {
        position: fixed;
        pointer-events: none;
        z-index: 999999;
    }
</style>
