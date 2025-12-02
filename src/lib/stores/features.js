import { writable } from "svelte/store";
import { listFeatures } from "$lib/commands.js";

function createFeatureStore() {
    const { subscribe, set, update } = writable([]);

    async function refresh() {
        const all = await listFeatures();
        set(all);
    }

    return {
        subscribe,
        refresh
    }
}

export const features = createFeatureStore();
