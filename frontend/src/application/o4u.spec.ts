import { describe, it, expect, vi, afterEach } from 'vitest';
import { O4u } from './o4u';
import { HttpRepository } from '../infrastructure/http-repository';
import { ApiFailure } from '../domain/entities';
import {
    toggleRegisterModule,
    mapRegisterFieldErrors,
} from '../domain/marketing/erp-saas-register';
const entity = {
    id: 1,
    name: 'ACME',
    uuid: 'a5efdbf5-0835-46a7-9926-d18a36bb8ac4',
    status: 'active',
};
afterEach(() => vi.unstubAllGlobals());
describe('O4U application and HTTP boundary', () => {
    it('rejects an empty name before sending a command', async () => {
        const fetch = vi.fn();
        vi.stubGlobal('fetch', fetch);
        const app = new O4u(new HttpRepository());
        await expect(
            app.save('clients', null, { name: '  ', status: 'active' }),
        ).rejects.toBeInstanceOf(ApiFailure);
        expect(fetch).not.toHaveBeenCalled();
    });
    it('loads every page for client choices', async () => {
        const fetch = vi
            .fn()
            .mockResolvedValueOnce(
                Response.json({
                    data: [entity],
                    total: 2,
                    current_page: 1,
                    last_page: 2,
                }),
            )
            .mockResolvedValueOnce(
                Response.json({
                    data: [{ ...entity, id: 2 }],
                    total: 2,
                    current_page: 2,
                    last_page: 2,
                }),
            );
        vi.stubGlobal('fetch', fetch);
        expect(
            await new O4u(new HttpRepository()).choices('clients'),
        ).toHaveLength(2);
        expect(fetch.mock.calls[1]?.[0]).toContain('page=2');
    });
    it('rejects malformed server records at runtime', async () => {
        vi.stubGlobal(
            'fetch',
            vi
                .fn()
                .mockResolvedValue(
                    Response.json({ ...entity, id: 'wrong type' }),
                ),
        );
        await expect(new HttpRepository().get('clients', 1)).rejects.toThrow();
    });
    it('sends JSON with session credentials and preserves validation errors', async () => {
        const fetch = vi.fn().mockResolvedValue(
            Response.json(
                {
                    message: 'Validation failed',
                    errors: { name: ['Required'] },
                },
                { status: 422 },
            ),
        );
        vi.stubGlobal('fetch', fetch);
        await expect(
            new HttpRepository().save('clients', null, {
                name: 'ACME',
                status: 'active',
            }),
        ).rejects.toMatchObject({
            status: 422,
            errors: { name: ['Required'] },
        });
        expect(fetch.mock.calls[0]?.[1]).toMatchObject({
            method: 'POST',
            credentials: 'include',
            body: JSON.stringify({ name: 'ACME', status: 'active' }),
        });
    });
    it('normalizes nullable fields from the existing PostgreSQL schema', async () => {
        vi.stubGlobal(
            'fetch',
            vi.fn().mockResolvedValue(
                Response.json({
                    ...entity,
                    name: null,
                    allow_public: null,
                    allow_get_info: null,
                }),
            ),
        );
        expect(await new HttpRepository().get('apps', 1)).toMatchObject({
            name: '',
            allow_public: false,
            allow_get_info: false,
        });
    });
    it('handles empty successful delete responses', async () => {
        vi.stubGlobal(
            'fetch',
            vi.fn().mockResolvedValue(new Response(null, { status: 204 })),
        );
        await expect(
            new HttpRepository().remove('clients', 1),
        ).resolves.toBeUndefined();
    });
    it('enforces the ERP module limit and supports deselection', () => {
        expect(toggleRegisterModule(['sale', 'purchase'], 'inventory')).toEqual(
            ['sale', 'purchase'],
        );
        expect(toggleRegisterModule(['sale'], 'sale')).toEqual([]);
        expect(mapRegisterFieldErrors({ email: ['Invalid', 'Other'] })).toEqual(
            { email: 'Invalid' },
        );
    });
});
