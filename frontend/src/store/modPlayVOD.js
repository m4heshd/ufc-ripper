// Store
import {defineStore} from 'pinia';

export const useModPlayVODStore = defineStore('modPlayVOD', {
    state: () => ({
        active: false,
        VOD: {}
    }),
    actions: {
        show(VOD) {
            if (VOD) this.setVOD(VOD);
            this.active = true;
        },
        close() {
            this.active = false;
        },
        setVOD(VOD) {
            Object.assign(this.VOD, VOD);
        }
    }
});
