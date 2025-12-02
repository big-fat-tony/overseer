<script>
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import { useFeatureSettings } from "$lib/features/useFeatureSettings.js";
    import RoleCard from "$lib/components/autopickban/RoleCard.svelte";

    const { settings, loaded, saveSetting } = useFeatureSettings("AutoPickBan");

    let role;
    let pick = {};
    let ban = {};

    $: role = $page.params.role;

    $: if ($loaded) {
        pick = $settings.pickPreferences ?? {};
        ban = $settings.banPreferences ?? {};
    }

    const updatePick = (role, data) => {
        pick = { ...pick, [role]: data };
        saveSetting("pickPreferences", pick);
    };

    const updateBan = (role, data) => {
        ban = { ...ban, [role]: data };
        saveSetting("banPreferences", ban);
    };

    const goBack = () => goto("/features/AutoPickBan");
</script>

{#if $loaded}
    <div class="page">

        <div class="topbar">
            <button class="back" on:click={goBack}>← Back</button>
            <h2>{role}</h2>
        </div>

        <div class="editor-wrapper">
            <RoleCard
                    role={role}
                    open={true}
                    picks={pick[role] ?? []}
                    bans={ban[role] ?? []}
                    on:updatePick={(e) => updatePick(role, e.detail)}
                    on:updateBan={(e) => updateBan(role, e.detail)}
            />
        </div>

    </div>
{/if}

<style>
    .page {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .topbar {
        display: flex;
        align-items: center;
        gap: 14px;
    }

    .back {
        color: var(--text);
        background: var(--surface);
        padding: 8px 14px;
        border-radius: 10px;
        font-size: .9rem;
        cursor: pointer;
        border: 1px solid var(--border);
    }

    .editor-wrapper {
        flex: 1;
        display: grid;
    }

    .editor-wrapper > :global(article) {
        height: 100%;
        display: flex;
        flex-direction: column;
    }
</style>
