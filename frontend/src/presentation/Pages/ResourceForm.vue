<template>
    <div class="admin-toolbar">
        <h1 class="text-h4">
            {{ id ? 'Chỉnh sửa' : 'Tạo' }} {{ title.toLowerCase() }}
        </h1>
        <v-btn :to="`/${resource}`" variant="text">Quay lại</v-btn>
    </div>
    <v-progress-linear v-if="loading" indeterminate /><v-alert
        v-if="error"
        type="error"
        class="mb-5"
        >{{ error }}</v-alert
    >
    <v-form v-if="!loading && ready" class="admin-form" @submit.prevent="save">
        <v-text-field
            v-model="draft.name"
            label="Tên"
            :error-messages="errors.name"
            maxlength="255"
            required
        />
        <v-select
            v-if="resource === 'client-systems'"
            v-model="draft.client_id"
            label="Khách hàng"
            :items="clients"
            item-title="name"
            item-value="id"
            :error-messages="errors.client_id"
        />
        <template v-if="resource === 'client-systems'"
            ><v-text-field
                v-model="draft.endpoint"
                label="Odoo endpoint"
                :error-messages="errors.endpoint"
                required /><v-text-field
                v-model="draft.webapp_endpoint"
                label="Webapp endpoint"
                :error-messages="errors.webapp_endpoint"
                clearable /><v-text-field
                v-model="draft.db_name"
                label="Database name"
                :error-messages="errors.db_name"
                required /><v-switch
                v-model="draft.allow_get_info"
                label="Cho phép lấy App Info"
                color="primary"
        /></template>
        <template v-if="resource === 'apps'"
            ><v-text-field
                v-for="key in settingKeys"
                :key="key"
                v-model="draft[key]"
                :label="settingLabels[key]"
                :error-messages="errors[key]"
                clearable /><v-switch
                v-model="draft.allow_public"
                label="Cho phép truy cập công khai"
                color="primary"
        /></template>
        <v-select
            :model-value="String(draft.status)"
            @update:model-value="draft.status = $event"
            label="Trạng thái"
            :items="statuses"
            :error-messages="errors.status"
        />
        <v-btn type="submit" color="primary" :loading="saving">Lưu</v-btn
        ><v-btn :to="`/${resource}`" class="ml-3" variant="text">Hủy</v-btn>
    </v-form>
</template>
<script setup lang="ts">
import { computed, onMounted, reactive, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useO4u } from '../../app/runtime';
import {
    ApiFailure,
    settingKeys,
    type Draft,
    type Entity,
} from '../../domain/entities';
import {
    useResource,
    settingLabels,
    errorMessage,
} from '../composables/resources';
const { resource, title } = useResource();
const o4u = useO4u();
const route = useRoute();
const router = useRouter();
const id = route.params.id ? Number(route.params.id) : null;
const draft = reactive<Draft>({
    name: '',
    status: 'active',
    allow_public: false,
    allow_get_info: false,
    client_id: null,
    endpoint: '',
    webapp_endpoint: null,
    db_name: '',
});
for (const key of settingKeys) draft[key] = null;
const statuses = computed(() => [
    'active',
    'inactive',
    ...(resource.value === 'apps'
        ? ['maintenance']
        : resource.value === 'client-systems'
          ? ['expired']
          : []),
]);
const loading = ref(true);
const saving = ref(false);
const ready = ref(false);
const error = ref('');
const errors = ref<Record<string, string[]>>({});
const clients = ref<Entity[]>([]);
onMounted(async () => {
    try {
        if (resource.value === 'client-systems')
            clients.value = await o4u.choices('clients');
        if (id !== null) {
            const entity = await o4u.repository.get(resource.value, id);
            for (const [key, value] of Object.entries(entity))
                if (
                    key in draft &&
                    (typeof value === 'string' ||
                        typeof value === 'number' ||
                        typeof value === 'boolean' ||
                        value === null)
                )
                    draft[key] = value;
        }
        ready.value = true;
    } catch (e) {
        error.value = errorMessage(e);
    } finally {
        loading.value = false;
    }
});
async function save() {
    saving.value = true;
    errors.value = {};
    error.value = '';
    try {
        await o4u.save(resource.value, id, draft);
        await router.push(`/${resource.value}`);
    } catch (e) {
        error.value = errorMessage(e);
        if (e instanceof ApiFailure) errors.value = e.errors;
    } finally {
        saving.value = false;
    }
}
</script>
