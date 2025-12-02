export function highlightJson(jsonString) {
    return jsonString
        .replace(/"(.*?)":/g, '<span class="key">"$1"</span>:')
        .replace(/"(.*?)"/g, '<span class="string">"$1"</span>')
        .replace(/\b(true|false)\b/g, '<span class="boolean">$1</span>')
        .replace(/\b(null)\b/g, '<span class="null">$1</span>')
        .replace(/\b(\d+)\b/g, '<span class="number">$1</span>');
}
