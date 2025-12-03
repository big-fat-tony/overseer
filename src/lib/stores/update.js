import { writable } from 'svelte/store'
import { invoke, Channel } from '@tauri-apps/api/core'

class UpdateService {
    check() {
        return invoke('check_update')
    }

    install(onEvent) {
        const channel = new Channel()
        channel.onmessage = onEvent
        return invoke('install_update', { onEvent: channel })
    }
}

class UpdateState {
    constructor() {
        this.service = new UpdateService()
        this.info = writable(null)
        this.installing = writable(false)
        this.progress = writable(0)
    }

    async check() {
        const info = await this.service.check()
        this.info.set(info)
    }

    async install() {
        this.installing.set(true)
        this.progress.set(0)

        await this.service.install(ev => {
            if (ev.event === 'Started') this.progress.set(0)
            if (ev.event === 'Progress') this.progress.update(x => x + ev.data.chunkLength)
            if (ev.event === 'Finished') this.progress.set(1)
        })
    }
}

export const update = new UpdateState()
