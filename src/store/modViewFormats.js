// Store
import {defineStore} from 'pinia';

export const useModViewFormatsStore = defineStore('modViewFormats', {
    state: () => ({
        vodData: {}
    }),
    actions: {
        setVODData(newData) {
            Object.assign(this.vodData, newData);
        }
    }
});
