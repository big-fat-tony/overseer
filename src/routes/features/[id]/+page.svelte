<script>
    import { listFeatures } from "$lib/commands.js";

    export let params;
    let feature = null;

    async function load() {
        const all = await listFeatures();
        feature = all.find((f) => f.id === params.id) ?? null;
    }

    load();
</script>

{#if feature}
    <section>
        <h1>{feature.name}</h1>
        <p>ID: {feature.id}</p>
        <p>Enabled: {feature.enabled ? "Yes" : "No"}</p>
        <p>(No custom UI implemented for this feature yet.)</p>
    </section>
{:else}
    <p>Unknown feature: {params.id}</p>
{/if}
