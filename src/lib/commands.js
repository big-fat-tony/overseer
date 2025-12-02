import { invoke } from "@tauri-apps/api/core";

export function listFeatures() {
    return invoke("list_features");
}

export function enableFeature(id) {
    return invoke("enable_feature", { id });
}

export function disableFeature(id) {
    return invoke("disable_feature", { id });
}

export function getFeatureSettings(id) {
    return invoke("get_feature_settings", { featureId: id });
}

export function setFeatureSetting(id, key, value) {
    return invoke("set_feature_setting", {
        featureId: id,
        key,
        value
    });
}

export function listChampions() {
    return invoke("list_champions");
}

export function saveRunePage(page) {
    return invoke("save_rune_page", { page });
}

export function listRunePages() {
    return invoke("list_rune_pages");
}

export function deleteRunePage(id) {
    return invoke("delete_rune_page", { id });
}
