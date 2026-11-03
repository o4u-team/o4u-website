<template>
    <div class="admin-toolbar">
        <h1 class="text-h4">Cấu hình app · {{ data?.clientSystem.name }}</h1>
        <v-btn to="/client-systems" variant="text">Quay lại</v-btn>
    </div>
    <p class="mb-6">
        Để trống từng trường để dùng cấu hình mặc định của ứng dụng.
    </p>
    <v-alert v-if="error" type="error" class="mb-5">{{ error }}</v-alert
    ><v-alert v-if="success" type="success" class="mb-5"
        >Đã lưu cấu hình.</v-alert
    ><v-progress-linear v-if="loading" indeterminate />
    <v-form v-if="data" @submit.prevent="save"
        ><v-expansion-panels multiple
            ><v-expansion-panel v-for="app in data.apps" :key="app.id"
                ><v-expansion-panel-title
                    >{{ app.name }}
                    <v-chip
                        v-if="selected.includes(app.id)"
                        size="small"
                        color="success"
                        class="ml-3"
                        >Đã cấp quyền</v-chip
                    ></v-expansion-panel-title
                ><v-expansion-panel-text
                    ><p class="text-caption mb-3">{{ app.uuid }}</p>
                    <v-switch
                        v-model="selected"
                        :value="app.id"
                        label="Cho phép sử dụng app"
                        color="primary"
                        :disabled="saving"
                    /><template
                        v-for="field in fieldsFor(app.id)"
                        :key="field.key"
                        ><v-text-field
                            v-model="field.value"
                            :label="settingLabels[field.key]"
                            :hint="`Mặc định: ${app[field.key] || 'Chưa cấu hình'}`"
                            persistent-hint
                            clearable
                            :disabled="!selected.includes(app.id) || saving"
                            :error-messages="
                                errors[`app_settings.${app.id}.${field.key}`]
                            " /></template
                    ><v-btn
                        variant="text"
                        :disabled="saving"
                        @click="reset(app.id)"
                        >Dùng toàn bộ mặc định</v-btn
                    ></v-expansion-panel-text
                ></v-expansion-panel
            ></v-expansion-panels
        >
        <p v-if="!data.apps.length" class="empty-state">Không có ứng dụng.</p>
        <v-btn type="submit" class="mt-6" color="primary" :loading="saving"
            >Lưu cấu hình</v-btn
        ></v-form
    >
</template>
<script setup lang="ts">
import { onMounted, ref, reactive } from 'vue';
import { useRoute } from 'vue-router';
import { useO4u } from '../../app/runtime';
import {
    ApiFailure,
    settingKeys,
    type Assignments,
    type SettingKey,
    type SyncApps,
} from '../../domain/entities';
import { settingLabels, errorMessage } from '../composables/resources';
const id = Number(useRoute().params.id);
const o4u = useO4u();
const data = ref<Assignments | null>(null);
const selected = ref<number[]>([]);
const loading = ref(true);
const saving = ref(false);
const error = ref('');
const success = ref(false);
const errors = ref<Record<string, string[]>>({});
const settings = reactive<
    Record<number, { key: SettingKey; value: string | null }[]>
>({});
function fieldsFor(id: number) {
    return settings[id] ?? [];
}
function reset(id: number) {
    for (const field of fieldsFor(id)) field.value = null;
}
onMounted(async () => {
    try {
        data.value = await o4u.repository.assignments(id);
        for (const app of data.value.apps) {
            if (app.assigned) selected.value.push(app.id);
            settings[app.id] = settingKeys.map((key) => ({
                key,
                value: app.overrides?.[key] ?? null,
            }));
        }
    } catch (e) {
        error.value = errorMessage(e);
    } finally {
        loading.value = false;
    }
});
async function save() {
    saving.value = true;
    success.value = false;
    error.value = '';
    errors.value = {};
    const payload: SyncApps = {
        app_ids: [...selected.value],
        app_settings: {},
    };
    for (const appId of selected.value) {
        const values: SyncApps['app_settings'][string] = {};
        for (const f of fieldsFor(appId)) values[f.key] = f.value || null;
        payload.app_settings[String(appId)] = values;
    }
    try {
        await o4u.repository.syncApps(id, payload);
        success.value = true;
    } catch (e) {
        error.value = errorMessage(e);
        if (e instanceof ApiFailure) errors.value = e.errors;
    } finally {
        saving.value = false;
    }
}
</script>
