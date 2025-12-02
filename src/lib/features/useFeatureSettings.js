import { writable } from "svelte/store";
import { getFeatureSettings, setFeatureSetting } from "$lib/commands.js";

export function useFeatureSettings(featureId) {
    const settings = writable({});
    const loaded = writable(false);

    async function load() {
        const s = await getFeatureSettings(featureId);
        settings.set(s);
        loaded.set(true);
    }

    function saveSetting(key, value) {
        setFeatureSetting(featureId, key, value);
        settings.update((old) => ({ ...old, [key]: value }));
    }

    load();

    return { settings, loaded, saveSetting };
}
