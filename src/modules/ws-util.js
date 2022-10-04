// Core
import {inject} from 'vue';

export function useWSUtil() {
    // Injects
    const {state} = inject('store');
    const socket = inject('socket');

    function emitPromise(event, ...args) {
        return new Promise((resolve, reject) => {
            socket.emit(event, ...args, (res) => res?.error ? reject(res.error) : resolve(res));
        });
    }

    async function saveConfig(newConfig) {
        await emitPromise('save-config', newConfig);
        state.config = JSON.parse(JSON.stringify(newConfig));
    }

    return {
        saveConfig
    };
}
