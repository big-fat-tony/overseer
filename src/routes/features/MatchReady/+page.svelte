<script>
    import { useFeatureSettings } from "$lib/features/useFeatureSettings.js"

    let delay = 0
    const { settings, loaded, saveSetting } = useFeatureSettings("MatchReady")

    $: if ($loaded) {
        delay = $settings.delayMs ?? 0
    }
</script>

{#if $loaded}
    <div class="settings-card">
        <label for="delay-slider" class="label">
            Auto-Accept Delay
        </label>

        <input
                id="delay-slider"
                type="range"
                min="0"
                max="11000"
                bind:value={delay}
                on:input={() => saveSetting("delayMs", delay)}
                class="slider"
        />

        <div class="value">
            {delay} ms
        </div>
    </div>
{/if}

<style>
    .settings-card {
        background: var(--surface);
        border-radius: var(--radius);
        box-shadow: var(--shadow);
        border: 1px solid var(--border);
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 20px;
        width: 320px;
        margin: 0 auto;
    }

    .label {
        font-weight: 600;
        font-size: 1.1rem;
        color: var(--text);
    }

    .value {
        font-size: 1.3rem;
        font-weight: 600;
        color: var(--text);
        text-align: center;
    }

    .slider {
        appearance: none;
        width: 100%;
        height: 6px;
        background: #ccd0d4;
        border-radius: 4px;
        outline: none;
        transition: background 0.2s;
    }

    .slider::-webkit-slider-thumb {
        appearance: none;
        width: 18px;
        height: 18px;
        border-radius: 50%;
        background: var(--accent);
        cursor: pointer;
        transition: transform 0.2s;
        box-shadow: 0 2px 6px rgba(0,0,0,0.15);
    }

    .slider::-webkit-slider-thumb:hover {
        transform: scale(1.1);
    }

    .slider::-moz-range-thumb {
        width: 18px;
        height: 18px;
        border-radius: 50%;
        background: var(--accent);
        cursor: pointer;
        box-shadow: 0 2px 6px rgba(0,0,0,0.15);
        transition: transform 0.2s;
    }

    .slider::-moz-range-thumb:hover {
        transform: scale(1.1);
    }
</style>
