import { inject, type InjectionKey, type Ref } from 'vue';
import type { O4u } from '../application/o4u';
import type { PublicConfig, User } from '../domain/entities';

export interface SessionStore {
    user: Ref<User | null>;
    load(): Promise<User | null>;
    logout(): Promise<void>;
}

export interface AppRuntime {
    o4u: O4u;
    session: SessionStore;
    publicConfig: PublicConfig;
}

export const appRuntimeKey: InjectionKey<AppRuntime> =
    Symbol('o4u-app-runtime');

export function useAppRuntime(): AppRuntime {
    const runtime = inject(appRuntimeKey);
    if (!runtime) throw new Error('O4U application runtime was not provided');
    return runtime;
}

export function useO4u(): O4u {
    return useAppRuntime().o4u;
}
