<script>
    import { onMount } from "svelte"
    import { enableFeature, disableFeature } from "$lib/commands.js"
    import { features } from "$lib/stores/features.js"
    import { clearLogs } from "$lib/stores/logs.js"
    import { logFeatureIds } from "$lib/features/constants.js"

    let loading = false

    onMount(() => {
        features.refresh()
    })

    async function toggle(feature, event) {
        const checked = event.currentTarget.checked
        loading = true

        try {
            if (checked) {
                await enableFeature(feature.id)
            } else {
                await disableFeature(feature.id)
                if (logFeatureIds.includes(feature.id)) clearLogs()
            }

            await features.refresh()
        } finally {
            loading = false
        }
    }
</script>

<section>
    <div class="features">
        {#each $features as feature}
            <div class="feature-item">
                <a href={`/features/${feature.id}`} class="name">
                    {feature.name}
                </a>

                <label class="switch">
                    <input
                            type="checkbox"
                            checked={feature.enabled}
                            disabled={loading}
                            on:change={(e) => toggle(feature, e)}
                    />
                    <span class="slider" />
                </label>
            </div>
        {/each}
    </div>
</section>

<style>
    section {
        padding: 4px;
    }

    .features {
        display: flex;
        flex-direction: column;
        gap: 14px;
    }

    .feature-item {
        background: var(--surface);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        padding: 16px 20px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        border: 1px solid var(--border);
    }

    .name {
        color: var(--text);
        font-weight: 600;
        font-size: 1.05rem;
        text-decoration: none;
        transition: opacity 0.15s;
    }

    .name:hover {
        opacity: 0.7;
    }

    .switch {
        position: relative;
        width: 44px;
        height: 24px;
        display: inline-block;
    }

    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .slider {
        position: absolute;
        cursor: pointer;
        inset: 0;
        background: #ccd0d4;
        border-radius: 24px;
        transition: 0.25s;
    }

    input:checked + .slider {
        background: var(--accent);
    }

    .slider::before {
        position: absolute;
        content: "";
        height: 20px;
        width: 20px;
        left: 2px;
        top: 2px;
        background: white;
        border-radius: 50%;
        transition: 0.25s;
        box-shadow: 0 1px 4px rgba(0,0,0,0.12);
    }

    input:checked + .slider::before {
        transform: translateX(20px);
    }
</style>
