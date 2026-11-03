import {
    createServer,
    type IncomingMessage,
    type ServerResponse,
} from 'node:http';
import { readFile, stat } from 'node:fs/promises';
import { createReadStream } from 'node:fs';
import { extname, resolve, sep } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { createServer as createViteServer, type ViteDevServer } from 'vite';
import type { User } from '../src/domain/entities.js';
interface InitialState {
    user: User | null;
    publicConfig: { turnstileSiteKey: string; turnstileBypass: boolean };
}
type Render = (
    url: string,
    state: InitialState,
    cookie: string,
    backendUrl: string,
) => Promise<{ html: string; url: string; user: User | null }>;

const root = resolve(fileURLToPath(new URL('..', import.meta.url)));
const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:8088';
const port = Number(process.env.SSR_PORT || process.env.PORT || 5173);
const host = process.env.SSR_HOST || '127.0.0.1';
const dev = process.env.NODE_ENV !== 'production';
const mime: Record<string, string> = {
    '.css': 'text/css; charset=utf-8',
    '.html': 'text/html; charset=utf-8',
    '.ico': 'image/x-icon',
    '.js': 'text/javascript; charset=utf-8',
    '.json': 'application/json; charset=utf-8',
    '.png': 'image/png',
    '.svg': 'image/svg+xml',
    '.webp': 'image/webp',
    '.woff': 'font/woff',
    '.woff2': 'font/woff2',
    '.jpg': 'image/jpeg',
    '.jpeg': 'image/jpeg',
};

function escapeJson(value: unknown): string {
    return JSON.stringify(value)
        .replace(/</g, '\\u003c')
        .replace(/>/g, '\\u003e')
        .replace(/&/g, '\\u0026');
}
async function publicConfig(): Promise<InitialState['publicConfig']> {
    try {
        const response = await fetch(backendUrl + '/api/public/config', {
            signal: AbortSignal.timeout(3000),
        });
        if (!response.ok) throw new Error('Public config unavailable');
        const value: unknown = await response.json();
        if (
            typeof value === 'object' &&
            value !== null &&
            typeof Reflect.get(value, 'turnstileSiteKey') === 'string' &&
            typeof Reflect.get(value, 'turnstileBypass') === 'boolean'
        ) {
            return {
                turnstileSiteKey: Reflect.get(
                    value,
                    'turnstileSiteKey',
                ) as string,
                turnstileBypass: Reflect.get(
                    value,
                    'turnstileBypass',
                ) as boolean,
            };
        }
    } catch {
        /* Fail closed while the API is unavailable. */
    }
    return { turnstileSiteKey: '', turnstileBypass: false };
}
async function serveFile(
    path: string,
    response: ServerResponse,
): Promise<boolean> {
    let decoded: string;
    try {
        decoded = decodeURIComponent(path);
    } catch {
        return false;
    }
    const publicRoot = resolve(root, 'dist');
    const file = resolve(publicRoot, '.' + decoded);
    if (file !== publicRoot && !file.startsWith(publicRoot + sep)) return false;
    try {
        if (!(await stat(file)).isFile()) return false;
        response.writeHead(200, {
            'Content-Type': mime[extname(file)] ?? 'application/octet-stream',
            'X-Content-Type-Options': 'nosniff',
        });
        createReadStream(file).pipe(response);
        return true;
    } catch {
        return false;
    }
}
async function renderRequest(
    request: IncomingMessage,
    response: ServerResponse,
    vite?: ViteDevServer,
): Promise<void> {
    const requestUrl = new URL(
        request.url || '/',
        'http://' + (request.headers.host || 'localhost'),
    );
    if (
        requestUrl.pathname.startsWith('/assets/') ||
        requestUrl.pathname.startsWith('/images/') ||
        requestUrl.pathname.startsWith('/policies/') ||
        requestUrl.pathname === '/favicon.ico'
    ) {
        if (await serveFile(requestUrl.pathname, response)) return;
        response.writeHead(404).end('Not found');
        return;
    }
    let template: string;
    let render: Render;
    if (vite) {
        template = await readFile(resolve(root, 'index.html'), 'utf-8');
        template = await vite.transformIndexHtml(requestUrl.pathname, template);
        ({ render } = (await vite.ssrLoadModule('/src/entry-server.ts')) as {
            render: Render;
        });
    } else {
        template = await readFile(resolve(root, 'dist/index.html'), 'utf-8');
        ({ render } = (await import(
            pathToFileURL(resolve(root, 'dist-ssr/entry-server.js')).href
        )) as { render: Render });
        const manifest = JSON.parse(
            await readFile(resolve(root, 'dist/.vite/manifest.json'), 'utf-8'),
        ) as Record<string, { css?: string[] }>;
        const links = (manifest['src/entry-client.ts']?.css ?? [])
            .map((file) => '<link rel=\"stylesheet\" href=\"/' + file + '\">')
            .join('');
        template = template.replace('</head>', links + '</head>');
    }
    const state: InitialState = {
        user: null,
        publicConfig: await publicConfig(),
    };
    const result = await render(
        requestUrl.pathname + requestUrl.search,
        state,
        request.headers.cookie || '',
        backendUrl,
    );
    if (result.url !== requestUrl.pathname + requestUrl.search) {
        response.writeHead(302, { Location: result.url }).end();
        return;
    }
    const html = template
        .replace('<!--ssr-outlet-->', result.html)
        .replace(
            '<!--ssr-state-->',
            escapeJson({ ...state, user: result.user }),
        );
    response.writeHead(200, {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'no-store',
        'X-Content-Type-Options': 'nosniff',
    });
    response.end(html);
}
let vite: ViteDevServer | undefined;
if (dev)
    vite = await createViteServer({
        configFile: resolve(root, 'vite.config.ts'),
        server: { middlewareMode: true },
        appType: 'custom',
    });
const server = createServer((request, response) => {
    if (vite) {
        vite.middlewares(request, response, (error?: Error | null) => {
            if (error) {
                response.writeHead(500).end('Internal server error');
                return;
            }
            void renderRequest(request, response, vite).catch((error) => {
                vite?.ssrFixStacktrace(error as Error);
                console.error(error);
                if (!response.headersSent)
                    response.writeHead(500, {
                        'Content-Type': 'text/plain; charset=utf-8',
                    });
                response.end('Unable to render page');
            });
        });
        return;
    }
    void renderRequest(request, response).catch((error) => {
        console.error(error);
        if (!response.headersSent)
            response.writeHead(500, {
                'Content-Type': 'text/plain; charset=utf-8',
            });
        response.end('Unable to render page');
    });
});
server.listen(port, host, () =>
    console.info('O4U Vue SSR listening at http://' + host + ':' + port),
);
for (const signal of ['SIGINT', 'SIGTERM'] as const)
    process.on(signal, () =>
        server.close(() => {
            void vite?.close();
            process.exit(0);
        }),
    );
