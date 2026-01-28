/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_APP_NAME: string
}

interface ImportMeta {
    readonly env: ImportMetaEnv
}

declare module '@inertiajs/core' {
    interface PageProps {
        auth?: {
            user?: {
                id: number;
                name: string;
                email: string;
                avatar: string | null;
            } | null;
        };
        flash?: {
            error?: string | null;
            success?: string | null;
        };
        [key: string]: any;
    }
}
