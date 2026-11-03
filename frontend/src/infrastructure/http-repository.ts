import { z } from 'zod';
import type { O4uRepository } from '../application/ports';
import {
    ApiFailure,
    type Draft,
    type ListFilters,
    type Resource,
    type SyncApps,
} from '../domain/entities';
import type { ErpSaasRegisterPayload } from '../domain/marketing/erp-saas-register';
const entity = z.object({
    id: z.number().int(),
    name: z
        .string()
        .nullable()
        .transform((value) => value ?? ''),
    uuid: z.string(),
    status: z
        .string()
        .nullable()
        .transform((value) => value ?? 'inactive'),
    created_at: z.string().nullable().optional(),
    updated_at: z.string().nullable().optional(),
    client_id: z.number().int().optional(),
    endpoint: z
        .string()
        .nullable()
        .optional()
        .transform((value) => value ?? ''),
    db_name: z
        .string()
        .nullable()
        .optional()
        .transform((value) => value ?? ''),
    webapp_endpoint: z.string().nullable().optional(),
    allow_public: z
        .boolean()
        .nullish()
        .transform((value) => value ?? false),
    allow_get_info: z
        .boolean()
        .nullish()
        .transform((value) => value ?? false),
    webapp_url: z.string().nullable().optional(),
    android_min_version: z.string().nullable().optional(),
    android_current_version: z.string().nullable().optional(),
    ios_min_version: z.string().nullable().optional(),
    ios_current_version: z.string().nullable().optional(),
    android_store_url: z.string().nullable().optional(),
    apple_store_url: z.string().nullable().optional(),
});
const row = entity.extend({ client: entity.nullable().optional() });
const settings = z.object({
    webapp_url: z.string().nullable().optional(),
    android_min_version: z.string().nullable().optional(),
    android_current_version: z.string().nullable().optional(),
    ios_min_version: z.string().nullable().optional(),
    ios_current_version: z.string().nullable().optional(),
    android_store_url: z.string().nullable().optional(),
    apple_store_url: z.string().nullable().optional(),
});
const user = z.object({
    id: z.number().int(),
    name: z.string(),
    email: z.string(),
    avatar: z.string().nullable(),
});
const page = <T extends z.ZodType>(item: T) =>
    z.object({
        data: z.array(item),
        total: z.number(),
        current_page: z.number(),
        last_page: z.number(),
    });
const device = z.object({
    id: z.number(),
    username: z.string(),
    app_version: z.string().nullable(),
    last_connected_at: z.string().nullable(),
    app: entity,
    client_system: entity,
    user_device: z.object({ device_id: z.string(), device_info: z.unknown() }),
});
const registration = z.object({
    success: z.boolean(),
    message: z.string(),
    code: z.string().nullable().optional(),
    errors: z.record(z.string(), z.array(z.string())).optional(),
    data: z
        .object({
            tenant_url: z.string().nullable(),
            email: z.string(),
            email_sent: z.boolean(),
            domain: z.string().nullable(),
        })
        .optional(),
});
const failure = z.object({
    message: z.string().optional(),
    errors: z.record(z.string(), z.array(z.string())).optional(),
});
function query(params: object): string {
    const out = new URLSearchParams();
    for (const [key, value] of Object.entries(params))
        if (value !== '' && value !== undefined && value !== null)
            out.set(key, String(value));
    return out.toString();
}
export class HttpRepository implements O4uRepository {
    constructor(
        private readonly base = '',
        private readonly cookie?: string,
    ) {}
    private async request(path: string, init: RequestInit = {}) {
        const response = await fetch(`${this.base}${path}`, {
            ...init,
            credentials: 'include',
            headers: {
                Accept: 'application/json',
                ...(init.body ? { 'Content-Type': 'application/json' } : {}),
                ...(this.cookie ? { Cookie: this.cookie } : {}),
                ...init.headers,
            },
        });
        const body: unknown =
            response.status === 204 ? null : await response.json();
        if (!response.ok) {
            const parsed = failure.safeParse(body);
            throw new ApiFailure(
                parsed.success
                    ? (parsed.data.message ?? 'Yêu cầu thất bại.')
                    : 'Phản hồi máy chủ không hợp lệ.',
                response.status,
                parsed.success ? parsed.data.errors : undefined,
            );
        }
        return body;
    }
    async me() {
        return user.parse(await this.request('/api/admin/me'));
    }
    async logout() {
        await this.request('/api/admin/logout', { method: 'POST' });
    }
    async list(resource: Resource, filters: ListFilters) {
        return page(row).parse(
            await this.request(`/api/admin/${resource}?${query(filters)}`),
        );
    }
    async get(resource: Resource, id: number) {
        return row.parse(await this.request(`/api/admin/${resource}/${id}`));
    }
    async save(resource: Resource, id: number | null, draft: Draft) {
        return row.parse(
            await this.request(
                `/api/admin/${resource}${id === null ? '' : `/${id}`}`,
                {
                    method: id === null ? 'POST' : 'PUT',
                    body: JSON.stringify(draft),
                },
            ),
        );
    }
    async remove(resource: Resource, id: number) {
        await this.request(`/api/admin/${resource}/${id}`, {
            method: 'DELETE',
        });
    }
    async assignments(id: number) {
        return z
            .object({
                clientSystem: entity,
                apps: z.array(
                    entity.extend({
                        assigned: z.boolean(),
                        overrides: settings.nullable(),
                    }),
                ),
            })
            .parse(await this.request(`/api/admin/client-systems/${id}/apps`));
    }
    async syncApps(id: number, data: SyncApps) {
        await this.request(`/api/admin/client-systems/${id}/sync-apps`, {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }
    async usage(filters: Record<string, string | number>) {
        return z
            .object({
                appUserDevices: page(device),
                connectionLogs: page(
                    z.object({
                        id: z.number(),
                        app_version: z.string().nullable(),
                        ip_address: z.string().nullable(),
                        created_at: z.string().nullable(),
                        app_user_device: device,
                    }),
                ),
                apps: z.array(entity),
                clientSystems: z.array(entity),
            })
            .parse(
                await this.request(`/api/admin/app-usage?${query(filters)}`),
            );
    }
    async publicConfig() {
        return z
            .object({
                turnstileSiteKey: z.string(),
                turnstileBypass: z.boolean(),
            })
            .parse(await this.request('/api/public/config'));
    }
    async register(data: ErpSaasRegisterPayload) {
        try {
            return registration.parse(
                await this.request('/erp-saas/register', {
                    method: 'POST',
                    body: JSON.stringify(data),
                }),
            );
        } catch (error) {
            if (error instanceof ApiFailure)
                return {
                    success: false,
                    message: error.message,
                    errors: error.errors,
                };
            throw error;
        }
    }
}
