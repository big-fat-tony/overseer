<script>
    import { createEventDispatcher } from "svelte";
    import PreferenceSection from "./PreferenceSection.svelte";
    import ChampionSearch from "./ChampionSearch.svelte";

    export let role;
    export let picks = [];
    export let bans = [];

    const dispatch = createEventDispatcher();

    const updatePicks = (e) => dispatch("updatePick", e.detail);
    const updateBans = (e) => dispatch("updateBan", e.detail);

    const addPick = (champ) => {
        const name = champ[0];
        if (!picks.includes(name)) dispatch("updatePick", [...picks, name]);
    };

    const addBan = (champ) => {
        const name = champ[0];
        if (!bans.includes(name)) dispatch("updateBan", [...bans, name]);
    };
</script>

<article class="role-card">
    <div class="role-header">
        <div class="role-meta">
            <div class="role-name">{role}</div>
            <div class="role-counts">
                <span>{picks.length} picks</span>
                <span>•</span>
                <span>{bans.length} bans</span>
            </div>
        </div>
    </div>

    <div class="layout">
        <div class="pick-area">
            <PreferenceSection
                    type="Pick"
                    data={picks}
                    add={addPick}
                    on:update={updatePicks}
            >
                <ChampionSearch
                        slot="input"
                        on:select={(e) => addPick(e.detail)}
                />
            </PreferenceSection>
        </div>

        <div class="ban-area">
            <PreferenceSection
                    type="Ban"
                    data={bans}
                    add={addBan}
                    on:update={updateBans}
            >
                <ChampionSearch
                        slot="input"
                        on:select={(e) => addBan(e.detail)}
                />
            </PreferenceSection>
        </div>
    </div>
</article>

<style>
    .role-card {
        background: var(--surface-translucent);
        backdrop-filter: blur(16px);
        border-radius: 18px;
        border: 1px solid var(--border);
        box-shadow: var(--shadow);
        overflow: hidden;
        display: flex;
        flex-direction: column;
        height: 100%;
    }

    .role-header {
        padding: 16px 20px;
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .role-meta {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .role-name {
        font-size: 1.1rem;
        font-weight: 700;
    }

    .role-counts {
        font-size: .8rem;
        opacity: .7;
        display: flex;
        gap: 6px;
        align-items: center;
    }

    .layout {
        flex: 1;
        display: grid;
        grid-template-areas: "pick ban";
        grid-template-columns: 1fr 1fr;
        gap: 24px;
        padding: 0 20px 20px 20px;
        overflow-y: auto;
    }

    .pick-area {
        grid-area: pick;
        display: flex;
        flex-direction: column;
    }

    .ban-area {
        grid-area: ban;
        display: flex;
        flex-direction: column;
    }

    @media(max-width: 700px) {
        .layout {
            grid-template-areas:
                "pick"
                "ban";
            grid-template-columns: 1fr;
        }
    }
</style>
