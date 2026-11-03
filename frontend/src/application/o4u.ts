import type { O4uRepository } from './ports';
import type { Draft, Resource } from '../domain/entities';
import { ApiFailure } from '../domain/entities';
export class O4u {
    constructor(public readonly repository: O4uRepository) {}
    save(resource: Resource, id: number | null, draft: Draft) {
        if (typeof draft.name !== 'string' || !draft.name.trim()) {
            return Promise.reject(
                new ApiFailure('Vui lòng nhập tên.', 422, {
                    name: ['Vui lòng nhập tên.'],
                }),
            );
        }
        return this.repository.save(resource, id, draft);
    }
    async choices(resource: Resource) {
        const all = [];
        for (let page = 1; ; page++) {
            const result = await this.repository.list(resource, {
                page,
                search: '',
                status: '',
                sort_field: 'name',
                sort_direction: 'asc',
            });
            all.push(...result.data);
            if (page >= result.last_page) return all;
        }
    }
}
