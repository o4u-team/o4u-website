import { defineConfig, loadEnv } from 'vite';
import vue from '@vitejs/plugin-vue';
import { fileURLToPath, URL } from 'node:url';
export default defineConfig(({ mode }) => {
    const env = loadEnv(mode, process.cwd(), '');
    const target = env.BACKEND_URL || 'http://127.0.0.1:8088';
    return {
        plugins: [vue()],
        ssr: { noExternal: ['vuetify'] },
        resolve: {
            alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) },
        },
        server: {
            middlewareMode: false,
            host: '127.0.0.1',
            port: 5173,
            strictPort: true,
            proxy: {
                '/api': target,
                '/auth': target,
                '/erp-saas/register': target,
            },
        },
        appType: 'custom',
        build: { outDir: 'dist', manifest: true },
    };
});
