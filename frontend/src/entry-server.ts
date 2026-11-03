import './presentation/style.css';
import { renderToString } from '@vue/server-renderer';
import { createO4uApp, type InitialState } from './app/create-app';

export async function render(
    url: string,
    state: InitialState,
    cookie: string,
    backendUrl: string,
) {
    const { app, router, session } = createO4uApp(true, state, {
        cookie,
        backendUrl,
    });
    await router.push(url);
    await router.isReady();
    const html = await renderToString(app);
    return {
        html,
        url: router.currentRoute.value.fullPath,
        user: session.user.value,
    };
}
