// Core
import {reactive} from "vue";

const state = reactive({
    config: {},
    ui: {
        overlay: true
    }
});

const actions = {
    showOverlay: () => state.ui.overlay = true,
    hideOverlay: () => state.ui.overlay = false
};

export {state, actions};
