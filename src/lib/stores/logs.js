import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import { onMount } from "svelte";

export const logs = writable([]);

let initialized = false;

export function initLogsListener() {
    if (initialized) return;
    initialized = true;

    onMount(async () => {
        const unlisten = await listen("log-entry", event => {
            logs.update(list => [...list, event.payload]);
        });

        return () => {
            unlisten();
            initialized = false;
        };
    });
}

export function clearLogs() {
    logs.set([]);
}
