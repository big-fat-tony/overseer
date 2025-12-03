import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/core'

class VersionState {
    constructor() {
        this.value = writable(null)
    }

    async load() {
        const v = await invoke('get_current_version')
        this.value.set(v)
    }
}

export const version = new VersionState()
