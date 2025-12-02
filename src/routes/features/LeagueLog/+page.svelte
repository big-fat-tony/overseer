<script>
    import { logs } from "$lib/stores/logs.js";
    import JsonLogViewer from "$lib/components/JsonLogViewer.svelte";

    $: leagueLogs = $logs.filter(l => l.source === "league");
</script>

<h1>League Event Log</h1>

<JsonLogViewer entries={leagueLogs} />
