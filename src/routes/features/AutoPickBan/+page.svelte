<script>
    import {ROLES} from "$lib/constants/roles.js";
    import {useFeatureSettings} from "$lib/features/useFeatureSettings.js";
    import {goto} from "$app/navigation";

    const {settings, loaded} = useFeatureSettings("AutoPickBan");

    let pick = {};
    let ban = {};

    $: if ($loaded) {
        pick = $settings.pickPreferences ?? {};
        ban = $settings.banPreferences ?? {};
    }

    const openRole = (role) => goto(`/features/AutoPickBan/${role}`);
</script>

{#if $loaded}
    <div class="page">

        <header class="header">
            <h2>Auto Pick / Ban</h2>
            <p>Select bans and picks for each role, the program will then pick and ban according to the assigned role in
                Champion Select, if there's no picks or bans available for the role the program will simply do
                nothing.</p>
        </header>

        <section class="roles-grid">
            {#each ROLES as role}
                <button class="role-tile" on:click={() => openRole(role)}>
                    <div class="role-name">{role}</div>

                    <div class="line">
                        <span class="label">Picks</span>
                        <div class="chips">
                            {#each (pick[role] ?? []).slice(0, 3) as p}
                                <span class="chip picks">{p}</span>
                            {/each}
                            {#if (pick[role]?.length ?? 0) > 3}
                                <span class="more">+{pick[role].length - 3}</span>
                            {/if}
                        </div>
                    </div>

                    <div class="line">
                        <span class="label">Bans</span>
                        <div class="chips">
                            {#each (ban[role] ?? []).slice(0, 3) as b}
                                <span class="chip bans">{b}</span>
                            {/each}
                            {#if (ban[role]?.length ?? 0) > 3}
                                <span class="more">+{ban[role].length - 3}</span>
                            {/if}
                        </div>
                    </div>
                </button>
            {/each}
        </section>

    </div>
{/if}

<style>
    .page {
        max-width: 1100px;
        margin: 0 auto;
        padding: 30px 20px;
        display: flex;
        flex-direction: column;
        gap: 30px;
    }

    .header h2 {
        margin: 0;
        font-size: 1.6rem;
    }

    .roles-grid {
        display: grid;
        gap: 20px;
        grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    }

    .role-tile {
        color: var(--text); /* ensure all text inside is light */
        background: linear-gradient(
                to bottom right,
                #1a2433,
                #11161e
        );
        border: 1px solid rgba(200, 170, 110, 0.25); /* subtle gold edge */
        border-radius: 16px;
        padding: 20px;
        text-align: left;
        cursor: pointer;
        display: flex;
        flex-direction: column;
        gap: 16px;
        box-shadow:
                0 4px 14px rgba(0,0,0,0.35),
                inset 0 0 14px rgba(0,0,0,0.35); /* inner vignette */
        transition:
                transform .18s ease,
                box-shadow .18s ease,
                border-color .18s ease,
                background .18s ease;
    }

    .role-tile:hover {
        transform: translateY(-3px);
        border-color: var(--accent); /* gold pop */
        background: linear-gradient(
                to bottom right,
                #223457,
                #161c25
        );
        box-shadow:
                0 10px 26px rgba(0,0,0,0.45),
                inset 0 0 18px rgba(0,0,0,0.25);
    }

    .role-name {
        font-size: 1.2rem;
        text-transform: capitalize;
        font-weight: 700;
        color: var(--accent);
        letter-spacing: .5px;
    }

    .line {
        display: flex;
        gap: 6px;
        align-items: center;
    }

    .label {
        font-size: .75rem;
        opacity: .7;
        width: 38px;
    }

    .chips {
        display: flex;
        gap: 6px;
        flex-wrap: wrap;
    }

    .chip {
        padding: 3px 8px;
        border-radius: 8px;
        border: 1px solid var(--border);
        background: var(--surface-translucent);
        font-size: .75rem;
    }


    .chip.picks {
        background: #1f4a1f;
    }

    .chip.bans {
        background: #512323;
    }

    .more {
        font-size: .75rem;
        opacity: .7;
    }
</style>
