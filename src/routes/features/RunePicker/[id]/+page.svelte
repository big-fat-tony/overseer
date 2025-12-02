<script>
    import { onMount } from "svelte"
    import { page } from "$app/stores"
    import { goto } from "$app/navigation"
    import RuneEditor from "$lib/components/runes/RuneEditor.svelte"
    import { listRunePages } from "$lib/commands.js"

    let params
    $: params = $page.params

    let existing = null

    onMount(async () => {
        const all = await listRunePages()
        existing = all.find(p => p.id === params.id)

        if (!existing) goto("/features/RunePicker")
    })
</script>

{#if existing}
    <RuneEditor mode="edit" existingPage={existing} />
{:else}
    <p>Loading...</p>
{/if}
