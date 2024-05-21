// Store
import {defineStore} from 'pinia';

export const useModPlayVODStore = defineStore('modPlayVOD', {
    state: () => ({
        active: false,
        VOD: {}
    }),
    actions: {
        showModPlayVOD() {
            this.active = true;
        },
        closeModPlayVOD() {
            this.active = false;
        },
        setVOD(VOD) {
            Object.assign(this.VOD, VOD);
        }
    }
});
