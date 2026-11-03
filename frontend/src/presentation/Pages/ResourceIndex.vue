<template>
    <div class="admin-toolbar">
        <h1 class="text-h4">{{ title }}</h1>
        <v-btn
            color="primary"
            :to="`/${resource}/create`"
            prepend-icon="mdi-plus"
            >Tạo mới</v-btn
        >
    </div>
    <form class="admin-toolbar" @submit.prevent="search">
        <v-text-field
            v-model="filters.search"
            label="Tìm kiếm tên, UUID, endpoint"
            hide-details
            clearable
            @click:clear="
                filters.search = '';
                search();
            "
        /><v-select
            v-model="filters.status"
            label="Trạng thái"
            :items="[
                '',
                'active',
                'inactive',
                ...(resource === 'apps'
                    ? ['maintenance']
                    : resource === 'client-systems'
                      ? ['expired']
                      : []),
            ]"
            hide-details
        /><v-select
            v-if="resource === 'client-systems'"
            v-model="filters.client_id"
            label="Khách hàng"
            :items="clients"
            item-title="name"
            item-value="id"
            clearable
            hide-details
        /><v-select
            v-model="filters.sort_field"
            label="Sắp xếp"
            :items="[
                { title: 'Ngày tạo', value: 'created_at' },
                { title: 'Tên', value: 'name' },
                { title: 'Trạng thái', value: 'status' },
            ]"
            hide-details
        /><v-btn type="submit" :loading="busy">Lọc</v-btn
        ><v-btn
            icon="mdi-sort"
            aria-label="Đảo chiều sắp xếp"
            @click="
                filters.sort_direction =
                    filters.sort_direction === 'asc' ? 'desc' : 'asc';
                search();
            "
        />
    </form>
    <v-alert v-if="error" type="error" class="mb-4">{{ error }}</v-alert
    ><v-progress-linear v-if="busy" indeterminate />
    <v-card variant="outlined"
        ><div class="table-scroll">
            <v-table
                ><thead>
                    <tr>
                        <th>Tên</th>
                        <th>UUID</th>
                        <th v-if="resource === 'client-systems'">
                            Endpoint / Database
                        </th>
                        <th>Trạng thái</th>
                        <th>Thao tác</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="row in result.data" :key="row.id">
                        <td>
                            {{ row.name }}
                            <div v-if="row.client" class="text-caption">
                                {{ row.client.name }}
                            </div>
                        </td>
                        <td class="text-caption">{{ row.uuid }}</td>
                        <td v-if="resource === 'client-systems'">
                            {{ row.endpoint }}<br />{{ row.db_name }}
                        </td>
                        <td>
                            <v-chip
                                size="small"
                                :color="
                                    row.status === 'active'
                                        ? 'success'
                                        : 'warning'
                                "
                                >{{ row.status }}</v-chip
                            >
                        </td>
                        <td>
                            <v-btn
                                :to="`/${resource}/${row.id}/edit`"
                                size="small"
                                variant="text"
                                >Sửa</v-btn
                            ><v-btn
                                v-if="resource === 'client-systems'"
                                :to="`/client-systems/${row.id}/app-info`"
                                size="small"
                                variant="text"
                                >Apps</v-btn
                            ><v-btn
                                color="error"
                                variant="text"
                                size="small"
                                @click="pending = row"
                                >Xóa</v-btn
                            >
                        </td>
                    </tr>
                </tbody></v-table
            >
        </div>
        <p v-if="!busy && !result.data.length" class="empty-state">
            Chưa có dữ liệu.
        </p></v-card
    >
    <div class="admin-toolbar mt-5">
        <span>{{ result.total }} bản ghi</span
        ><v-pagination
            v-model="filters.page"
            :length="result.last_page"
            :disabled="busy"
            @update:model-value="load"
        />
    </div>
    <v-dialog
        :model-value="pending !== null"
        max-width="460"
        @update:model-value="pending = null"
        ><v-card class="pa-6"
            ><h2 class="text-h6">Xóa {{ pending?.name }}?</h2>
            <p class="my-4">
                Dữ liệu liên quan có thể bị xóa theo ràng buộc của database.
            </p>
            <v-card-actions
                ><v-btn :disabled="deleting" @click="pending = null">Hủy</v-btn
                ><v-btn color="error" :loading="deleting" @click="remove"
                    >Xóa</v-btn
                ></v-card-actions
            ></v-card
        ></v-dialog
    >
</template>
<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { useO4u } from '../../app/runtime';
import type { Entity, ListFilters, Page } from '../../domain/entities';
import { useResource, errorMessage } from '../composables/resources';
const { resource, title } = useResource();
const o4u = useO4u();
const busy = ref(false);
const deleting = ref(false);
const pending = ref<Entity | null>(null);
const error = ref('');
const clients = ref<Entity[]>([]);
const filters = reactive<ListFilters>({
    search: '',
    status: '',
    page: 1,
    sort_field: 'created_at',
    sort_direction: 'desc',
});
const result = ref<Page<Entity>>({
    data: [],
    total: 0,
    current_page: 1,
    last_page: 1,
});
async function load() {
    busy.value = true;
    error.value = '';
    try {
        result.value = await o4u.repository.list(resource.value, filters);
    } catch (e) {
        error.value = errorMessage(e);
    } finally {
        busy.value = false;
    }
}
function search() {
    filters.page = 1;
    void load();
}
async function remove() {
    if (!pending.value) return;
    deleting.value = true;
    try {
        await o4u.repository.remove(resource.value, pending.value.id);
        pending.value = null;
        if (result.value.data.length === 1 && filters.page > 1) filters.page--;
        await load();
    } catch (e) {
        error.value = errorMessage(e);
        pending.value = null;
    } finally {
        deleting.value = false;
    }
}
onMounted(async () => {
    await load();
    if (resource.value === 'client-systems') {
        try {
            clients.value = await o4u.choices('clients');
        } catch (e) {
            error.value = errorMessage(e);
        }
    }
});
</script>
