import { createInertiaApp } from '@inertiajs/vue3';
import { createApp, DefineComponent, h } from 'vue';
import vuetify from './plugins/vuetify';

createInertiaApp({
    resolve: (name: string) => {
        const pages = import.meta.glob<{ default: DefineComponent }>('./Pages/**/*.vue', { eager: true });
        return pages[`./Pages/${name}.vue`]?.default ?? pages[`./Pages/${name}.vue`];
    },
    setup({ el, App, props, plugin }) {
        return createApp({ render: () => h(App, props) })
            .use(plugin)
            .use(vuetify)
            .mount(el)
    },
})
