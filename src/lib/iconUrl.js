export const iconUrl = (path) => {
    if (!path) return "";

    // Local shard assets (or any path starting with '/')
    if (path.startsWith("/")) {
        return path;
    }

    // Remote runes from DDragon
    return `https://ddragon.canisback.com/img/${path}`;
};
