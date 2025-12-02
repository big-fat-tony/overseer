<script>
    import { listFeatures } from "$lib/commands.js";
    import FeatureListItem from "./FeatureListItem.svelte";

    let features = [];

    async function load() {
        features = await listFeatures();
    }

    load();
</script>

<section>
    <h2>Features</h2>

    {#each features as feature}
        <FeatureListItem {feature} />
    {/each}
</section>
