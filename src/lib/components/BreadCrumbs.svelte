<script>
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { listRunePages } from "$lib/commands.js";

    let runePages = [];

    onMount(async () => {
        runePages = await listRunePages().catch(() => []);
    });

    $: segments = $page.url.pathname
        .split("/")
        .filter(Boolean)
        .map((s, i, arr) => {
            const href = "/" + arr.slice(0, i + 1).join("/");
            let label = s.replace(/-/g, " ");

            label = label.charAt(0).toUpperCase() + label.slice(1);

            // Rune page special-case: replace UUID with rune name
            if (arr[1] === "RunePicker" && i === 2) {
                const match = runePages.find(p => p.id === s);
                if (match) label = match.name;
            }

            return { label, href, isLast: i === arr.length - 1 };
        });
</script>

<nav class="navbar">
    {#each segments as seg, i}
        {#if seg.isLast}
            <span class="crumb current">{seg.label}</span>
        {:else}
            <a class="crumb link" href={seg.href}>{seg.label}</a>
            <span class="sep">/</span>
        {/if}
    {/each}
</nav>

<style>
    .navbar {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 10px 16px;
        background: #111;
        border-bottom: 1px solid #333;
        font-size: 0.95rem;
    }

    .crumb {
        color: #fff;
        text-decoration: none;
        white-space: nowrap;
        opacity: 0.9;
    }

    .crumb.link:hover {
        text-decoration: underline;
        opacity: 1;
    }

    .crumb.current {
        opacity: 0.6;
        cursor: default;
    }

    .sep {
        color: #666;
        opacity: 0.8;
    }
</style>
