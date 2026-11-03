import { createSSRApp, h, ref, type App } from 'vue';
import {
    createMemoryHistory,
    createRouter,
    createWebHistory,
} from 'vue-router';
import type { PublicConfig, User } from '../domain/entities';
import type { SessionStore } from './runtime';
import { appRuntimeKey } from './runtime';
import { createO4u } from './services';
import { createVuetifyApp } from './vuetify';
import Root from './App.vue';

const routes = [
    {
        path: '/',
        component: () => import('../presentation/Layouts/LandingLayout.vue'),
        children: [
            {
                path: '',
                component: () => import('../presentation/Pages/Welcome.vue'),
            },
            {
                path: 'erp-saas',
                component: () => import('../presentation/Pages/ErpSaas.vue'),
            },
        ],
    },
    {
        path: '/login',
        component: () => import('../presentation/Pages/Login.vue'),
    },
    {
        path: '/',
        component: () => import('../presentation/Layouts/AdminLayout.vue'),
        meta: { requiresAuth: true },
        children: [
            {
                path: 'dashboard',
                component: () => import('../presentation/Pages/Dashboard.vue'),
            },
            {
                path: 'app-usage',
                component: () => import('../presentation/Pages/AppUsage.vue'),
            },
            {
                path: 'client-systems/:id/app-info',
                component: () =>
                    import('../presentation/Pages/AppAssignments.vue'),
            },
            {
                path: ':resource(clients|apps|client-systems)',
                component: () =>
                    import('../presentation/Pages/ResourceIndex.vue'),
            },
            {
                path: ':resource(clients|apps|client-systems)/create',
                component: () =>
                    import('../presentation/Pages/ResourceForm.vue'),
            },
            {
                path: ':resource(clients|apps|client-systems)/:id/edit',
                component: () =>
                    import('../presentation/Pages/ResourceForm.vue'),
            },
        ],
    },
    {
        path: '/:policy(kho-barcode|cong-dat-hang|hr-cham-cong|approval)',
        component: () => import('../presentation/Pages/Policy.vue'),
    },
    {
        path: '/:pathMatch(.*)*',
        component: () => import('../presentation/Pages/NotFound.vue'),
    },
];

export interface InitialState {
    user: User | null;
    publicConfig: PublicConfig;
}

export interface CreatedApp {
    app: App;
    router: ReturnType<typeof createRouter>;
    session: SessionStore;
}

export function createO4uApp(
    server: boolean,
    initial: InitialState,
    options: { backendUrl?: string; cookie?: string } = {},
): CreatedApp {
    const base = server
        ? (options.backendUrl ?? '')
        : (import.meta.env.VITE_API_BASE_URL ?? '');
    const o4u = createO4u(base, server ? options.cookie : undefined);
    const user = ref<User | null>(initial.user);
    let userRequest: Promise<User | null> | null = null;
    const session: SessionStore = {
        user,
        async load() {
            userRequest ??= o4u.repository
                .me()
                .then((value) => {
                    user.value = value;
                    return value;
                })
                .catch((error: unknown) => {
                    userRequest = null;
                    if (
                        error instanceof Error &&
                        'status' in error &&
                        error.status === 401
                    ) {
                        user.value = null;
                        return null;
                    }
                    throw error;
                });
            return userRequest;
        },
        async logout() {
            await o4u.repository.logout();
            user.value = null;
            userRequest = null;
        },
    };
    const router = createRouter({
        history: server ? createMemoryHistory() : createWebHistory(),
        scrollBehavior(to) {
            return to.hash ? { el: to.hash, behavior: 'smooth' } : { top: 0 };
        },
        routes,
    });
    router.beforeEach(async (to) => {
        if (to.meta.requiresAuth) {
            try {
                if (!(await session.load()))
                    return { path: '/login', query: { redirect: to.fullPath } };
            } catch {
                return {
                    path: '/login',
                    query: { error: 'connection_failed' },
                };
            }
        }
        return true;
    });
    const app = createSSRApp({ render: () => h(Root) });
    app.provide(appRuntimeKey, {
        o4u,
        session,
        publicConfig: initial.publicConfig,
    });
    app.use(router);
    app.use(createVuetifyApp());
    return { app, router, session };
}
