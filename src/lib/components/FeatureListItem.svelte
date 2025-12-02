<script>
    import FeatureToggle from "./FeatureToggle.svelte";

    export let feature; // { id, name, enabled }
</script>

<div class="feature-row">
    <a href={`/features/${feature.id}`} class="name">
        {feature.name}
    </a>

    <FeatureToggle id={feature.id} enabled={feature.enabled} />
</div>

<style>
    .feature-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px 0;
    }

    .name {
        text-decoration: none;
        color: inherit;
    }
</style>
