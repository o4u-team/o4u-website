import './presentation/style.css';
import { createO4uApp, type InitialState } from './app/create-app';

const stateElement = document.getElementById('__O4U_SSR_STATE__');
let initial: InitialState = {
    user: null,
    publicConfig: { turnstileSiteKey: '', turnstileBypass: false },
};
if (stateElement?.textContent) {
    try {
        initial = JSON.parse(stateElement.textContent) as InitialState;
    } catch {
        stateElement.remove();
    }
}
stateElement?.remove();
const { app, router } = createO4uApp(false, initial);
await router.isReady();
app.mount('#app');
