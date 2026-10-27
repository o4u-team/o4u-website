<template>
    <v-container>
        <Head :title="`App Info · ${clientSystem.name}`" />
        <div class="d-flex align-center ga-3 mb-4">
            <v-btn icon="mdi-arrow-left" variant="text" aria-label="Quay lại Client Systems" @click="router.visit('/client-systems')" />
            <div>
                <h1 class="text-h5">Client System App Info</h1>
                <div class="text-subtitle-1">{{ clientSystem.name }}</div>
                <div class="text-caption text-medium-emphasis">{{ clientSystem.endpoint }} · {{ clientSystem.db_name }}</div>
            </div>
        </div>

        <v-alert v-if="page.props.flash?.success" type="success" variant="tonal" class="mb-4">
            {{ page.props.flash.success }}
        </v-alert>
        <p class="text-body-2 text-medium-emphasis mb-4">
            Mở từng app để cấp quyền và cấu hình riêng. Để trống từng trường để dùng giá trị mặc định của app.
        </p>
        <v-alert v-if="Object.keys(form.errors).length" type="error" variant="tonal" class="mb-4">
            Vui lòng kiểm tra các app có lỗi bên dưới.
        </v-alert>
        <form @submit.prevent="save">
            <v-expansion-panels v-model="openedApps" multiple variant="accordion">
                <v-expansion-panel v-for="app in apps" :key="app.id" :value="app.id">
                    <v-expansion-panel-title>
                        <div class="d-flex align-center flex-wrap ga-2">
                            <v-icon>mdi-application</v-icon>
                            <span>{{ app.name }}</span>
                            <v-chip v-if="form.app_ids.includes(app.id)" size="small" color="success" variant="tonal">Đã cấp quyền</v-chip>
                            <v-chip v-if="hasCustom(app.id)" size="small" color="warning" variant="tonal">Có cấu hình riêng</v-chip>
                            <v-chip v-if="hasErrors(app.id)" size="small" color="error" variant="tonal">Cần kiểm tra</v-chip>
                        </div>
                    </v-expansion-panel-title>
                    <v-expansion-panel-text>
                        <div class="text-caption text-medium-emphasis mb-3">{{ app.uuid }}</div>
                        <v-switch
                            v-model="form.app_ids"
                            :value="app.id"
                            label="Cho phép Client System sử dụng app"
                            color="primary"
                            hide-details
                            :disabled="form.processing"
                            class="mb-4"
                        />
                        <v-row>
                            <v-col v-for="field in fields" :key="field.key" cols="12" :md="field.key.includes('version') ? 6 : 12">
                                <v-text-field
                                    v-model="form.app_settings[app.id][field.key]"
                                    :label="field.label"
                                    :placeholder="app[`default_${field.key}`] || field.placeholder"
                                    :hint="`Mặc định: ${app[`default_${field.key}`] || 'Chưa cấu hình'}`"
                                    persistent-hint
                                    :error-messages="form.errors[`app_settings.${app.id}.${field.key}`]"
                                    :disabled="!form.app_ids.includes(app.id) || form.processing"
                                    :maxlength="field.key === 'webapp_url' || field.key.includes('version') ? 255 : 500"
                                    variant="outlined"
                                    density="comfortable"
                                    clearable
                                />
                            </v-col>
                        </v-row>
                        <v-btn
                            variant="text"
                            color="primary"
                            :disabled="!form.app_ids.includes(app.id) || form.processing || !hasCustom(app.id)"
                            @click="resetApp(app.id)"
                        >Dùng toàn bộ mặc định</v-btn>
                    </v-expansion-panel-text>
                </v-expansion-panel>
            </v-expansion-panels>
            <v-alert v-if="!apps.length" type="info" variant="tonal">Không có app để cấu hình.</v-alert>
            <div class="d-flex justify-end ga-2 mt-4">
                <v-btn variant="text" :disabled="form.processing" @click="router.visit('/client-systems')">Quay lại</v-btn>
                <v-btn v-if="apps.length" type="submit" color="primary" :loading="form.processing" prepend-icon="mdi-content-save">Lưu cấu hình</v-btn>
            </div>
        </form>
    </v-container>
</template>

<script setup lang="ts">
import DefaultLayout from '@/Layouts/DefaultLayout.vue';
import { Head, router, useForm, usePage } from '@inertiajs/vue3';
import { ref } from 'vue';

defineOptions({ layout: DefaultLayout });

interface AppInfo {
    id: number;
    name: string;
    uuid: string;
    assigned: boolean;
    [key: string]: any;
}

const props = defineProps<{
    clientSystem: { id: number; name: string; uuid: string; endpoint: string; db_name: string };
    apps: AppInfo[];
}>();
const page = usePage<{ flash?: { success?: string } }>();
const fields = [
    { key: 'webapp_url', label: 'WebApp URL riêng', placeholder: 'https://...' },
    { key: 'android_min_version', label: 'Android phiên bản tối thiểu', placeholder: '1.0.0' },
    { key: 'android_current_version', label: 'Android phiên bản hiện tại', placeholder: '1.0.0' },
    { key: 'android_store_url', label: 'Google Play URL', placeholder: 'https://play.google.com/...' },
    { key: 'ios_min_version', label: 'iOS phiên bản tối thiểu', placeholder: '1.0.0' },
    { key: 'ios_current_version', label: 'iOS phiên bản hiện tại', placeholder: '1.0.0' },
    { key: 'apple_store_url', label: 'App Store URL', placeholder: 'https://apps.apple.com/...' },
];
const openedApps = ref<number[]>([]);
const form = useForm({
    app_ids: props.apps.filter(app => app.assigned).map(app => app.id),
    app_settings: Object.fromEntries(props.apps.map(app => [app.id,
        Object.fromEntries(fields.map(field => [field.key, app[field.key] ?? null])),
    ])) as Record<number, Record<string, string | null>>,
});
const hasCustom = (id: number) => fields.some(field => (form.app_settings[id][field.key] || '').trim() !== '');
const hasErrors = (id: number) => Object.keys(form.errors).some(key => key.startsWith(`app_settings.${id}.`));
const resetApp = (id: number) => fields.forEach(field => { form.app_settings[id][field.key] = null; });
const save = () => {
    form.transform(data => ({
        ...data,
        app_settings: Object.fromEntries(Object.entries(data.app_settings).filter(([id]) => data.app_ids.includes(Number(id)))),
    })).post(`/client-systems/${props.clientSystem.id}/sync-apps`, {
        preserveScroll: true,
        onError: () => {
            openedApps.value = props.apps.filter(app => hasErrors(app.id)).map(app => app.id);
        },
    });
};
</script>
