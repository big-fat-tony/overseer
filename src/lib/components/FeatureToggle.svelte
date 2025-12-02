<script>
    import { enableFeature, disableFeature } from "$lib/commands.js";

    export let id;
    export let enabled;

    let checked = enabled;

    async function toggle(event) {
        checked = event.currentTarget.checked;

        if (checked) {
            await enableFeature(id);
        } else {
            await disableFeature(id);
        }
    }
</script>

<label>
    <input
            type="checkbox"
            bind:checked={checked}
            on:change={toggle}
    />
</label>
