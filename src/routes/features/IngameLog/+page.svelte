<script>
    import { logs } from "$lib/stores/logs.js";
    import JsonLogViewer from "$lib/components/JsonLogViewer.svelte";

    $: ingameLogs = $logs.filter(l => l.source === "ingame");
</script>

<h1>In-Game Event Log</h1>

<JsonLogViewer entries={ingameLogs} />
