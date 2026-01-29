<template>
    <v-container>
        <v-row justify="center">
            <v-col cols="12" md="10" lg="8">
                <v-card>
                    <v-card-title class="d-flex align-center">
                        <v-btn
                            icon
                            variant="text"
                            @click="router.visit('/apps')"
                            class="mr-2"
                        >
                            <v-icon>mdi-arrow-left</v-icon>
                            <v-tooltip activator="parent" location="bottom">Quay lại</v-tooltip>
                        </v-btn>
                        <v-icon class="mr-2">
                            {{ isEdit ? 'mdi-pencil' : 'mdi-plus' }}
                        </v-icon>
                        {{ isEdit ? 'Chỉnh sửa thông tin App' : 'Tạo App mới' }}
                    </v-card-title>

                    <v-divider></v-divider>

                    <v-card-text>
                        <v-form @submit.prevent="submitForm">
                            <v-row>
                                <!-- Name -->
                                <v-col cols="12">
                                    <v-text-field
                                        v-model="form.name"
                                        label="Tên App *"
                                        prepend-inner-icon="mdi-application"
                                        variant="outlined"
                                        :error-messages="errors.name"
                                        required
                                    ></v-text-field>
                                </v-col>

                                <!-- Android Versions -->
                                <v-col cols="12">
                                    <v-card variant="outlined" color="success">
                                        <v-card-title class="text-subtitle-1 d-flex align-center">
                                            <v-icon class="mr-2">mdi-android</v-icon>
                                            Android Versions
                                        </v-card-title>
                                        <v-card-text>
                                            <v-row>
                                                <v-col cols="12" md="6">
                                                    <v-text-field
                                                        v-model="form.android_min_version"
                                                        label="Min Version"
                                                        prepend-inner-icon="mdi-arrow-down-bold"
                                                        variant="outlined"
                                                        placeholder="1.0.0"
                                                        :error-messages="errors.android_min_version"
                                                        hint="Format: major.minor.patch (e.g., 1.0.0)"
                                                        persistent-hint
                                                        density="compact"
                                                        @blur="validateVersions"
                                                    ></v-text-field>
                                                </v-col>
                                                <v-col cols="12" md="6">
                                                    <v-text-field
                                                        v-model="form.android_current_version"
                                                        label="Current Version"
                                                        prepend-inner-icon="mdi-arrow-up-bold"
                                                        variant="outlined"
                                                        placeholder="1.0.0"
                                                        :error-messages="errors.android_current_version"
                                                        hint="Format: major.minor.patch (e.g., 1.0.0)"
                                                        persistent-hint
                                                        density="compact"
                                                        @blur="validateVersions"
                                                    ></v-text-field>
                                                </v-col>
                                            </v-row>
                                        </v-card-text>
                                    </v-card>
                                </v-col>

                                <!-- iOS Versions -->
                                <v-col cols="12">
                                    <v-card variant="outlined">
                                        <v-card-title class="text-subtitle-1 d-flex align-center">
                                            <v-icon class="mr-2">mdi-apple</v-icon>
                                            iOS Versions
                                        </v-card-title>
                                        <v-card-text>
                                            <v-row>
                                                <v-col cols="12" md="6">
                                                    <v-text-field
                                                        v-model="form.ios_min_version"
                                                        label="Min Version"
                                                        prepend-inner-icon="mdi-arrow-down-bold"
                                                        variant="outlined"
                                                        placeholder="1.0.0"
                                                        :error-messages="errors.ios_min_version"
                                                        hint="Format: major.minor.patch (e.g., 1.0.0)"
                                                        persistent-hint
                                                        density="compact"
                                                        @blur="validateVersions"
                                                    ></v-text-field>
                                                </v-col>
                                                <v-col cols="12" md="6">
                                                    <v-text-field
                                                        v-model="form.ios_current_version"
                                                        label="Current Version"
                                                        prepend-inner-icon="mdi-arrow-up-bold"
                                                        variant="outlined"
                                                        placeholder="1.0.0"
                                                        :error-messages="errors.ios_current_version"
                                                        hint="Format: major.minor.patch (e.g., 1.0.0)"
                                                        persistent-hint
                                                        density="compact"
                                                        @blur="validateVersions"
                                                    ></v-text-field>
                                                </v-col>
                                            </v-row>
                                        </v-card-text>
                                    </v-card>
                                </v-col>

                                <!-- Android Store URL -->
                                <v-col cols="12">
                                    <v-text-field
                                        v-model="form.android_store_url"
                                        label="Android Store URL"
                                        prepend-inner-icon="mdi-android"
                                        variant="outlined"
                                        placeholder="https://play.google.com/store/apps/details?id=..."
                                        :error-messages="errors.android_store_url"
                                    ></v-text-field>
                                </v-col>

                                <!-- Apple Store URL -->
                                <v-col cols="12">
                                    <v-text-field
                                        v-model="form.apple_store_url"
                                        label="Apple Store URL"
                                        prepend-inner-icon="mdi-apple"
                                        variant="outlined"
                                        placeholder="https://apps.apple.com/app/..."
                                        :error-messages="errors.apple_store_url"
                                    ></v-text-field>
                                </v-col>

                                <!-- Status -->
                                <v-col cols="12" md="6">
                                    <v-select
                                        v-model="form.status"
                                        :items="statusOptions"
                                        label="Trạng thái *"
                                        prepend-inner-icon="mdi-check-circle"
                                        variant="outlined"
                                        :error-messages="errors.status"
                                        required
                                    ></v-select>
                                </v-col>

                                <!-- UUID (read-only when editing) -->
                                <v-col v-if="isEdit && app?.uuid" cols="12">
                                    <v-alert
                                        type="info"
                                        variant="tonal"
                                    >
                                        <div class="text-subtitle-2 mb-1">UUID (không thể thay đổi)</div>
                                        <code class="text-body-2">{{ app.uuid }}</code>
                                    </v-alert>
                                </v-col>

                                <v-col cols="12">
                                    <div class="text-caption text-medium-emphasis">
                                        * Trường bắt buộc
                                    </div>
                                </v-col>
                            </v-row>
                        </v-form>
                    </v-card-text>

                    <v-divider></v-divider>

                    <v-card-actions class="pa-4">
                        <v-btn
                            variant="text"
                            @click="router.visit('/apps')"
                            :disabled="processing"
                        >
                            Hủy
                        </v-btn>
                        <v-spacer></v-spacer>
                        <v-btn
                            color="primary"
                            variant="flat"
                            @click="submitForm"
                            :loading="processing"
                            prepend-icon="mdi-content-save"
                        >
                            {{ isEdit ? 'Cập nhật' : 'Tạo mới' }}
                        </v-btn>
                    </v-card-actions>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue';
import { router } from '@inertiajs/vue3';
import DefaultLayout from '@/Layouts/DefaultLayout.vue';

interface AppItem {
    id: number;
    name: string;
    uuid: string;
    android_min_version: string | null;
    android_current_version: string | null;
    ios_min_version: string | null;
    ios_current_version: string | null;
    android_store_url: string | null;
    apple_store_url: string | null;
    status: 'active' | 'maintenance' | 'inactive';
}

interface Props {
    app?: AppItem | null;
}

defineOptions({
    layout: DefaultLayout,
});

const props = defineProps<Props>();

const isEdit = computed(() => !!props.app);

const processing = ref(false);

const form = reactive({
    name: props.app?.name || '',
    android_min_version: props.app?.android_min_version || '',
    android_current_version: props.app?.android_current_version || '',
    ios_min_version: props.app?.ios_min_version || '',
    ios_current_version: props.app?.ios_current_version || '',
    android_store_url: props.app?.android_store_url || '',
    apple_store_url: props.app?.apple_store_url || '',
    status: props.app?.status || 'active',
});

const errors = reactive<Record<string, string>>({
    name: '',
    android_min_version: '',
    android_current_version: '',
    ios_min_version: '',
    ios_current_version: '',
    android_store_url: '',
    apple_store_url: '',
    status: '',
});

const statusOptions = [
    { title: 'Hoạt động', value: 'active' },
    { title: 'Bảo trì', value: 'maintenance' },
    { title: 'Không hoạt động', value: 'inactive' },
];

/**
 * Validate semantic version format
 */
const isValidSemanticVersion = (version: string): boolean => {
    if (!version) return true; // empty is ok
    return /^\d+\.\d+\.\d+$/.test(version);
};

/**
 * Compare two semantic versions
 * Returns: -1 if v1 < v2, 0 if equal, 1 if v1 > v2
 */
const compareVersions = (v1: string, v2: string): number => {
    if (!v1 || !v2) return 0;

    const parts1 = v1.split('.').map(Number);
    const parts2 = v2.split('.').map(Number);

    for (let i = 0; i < 3; i++) {
        if (parts1[i] > parts2[i]) return 1;
        if (parts1[i] < parts2[i]) return -1;
    }

    return 0;
};

/**
 * Validate version fields
 */
const validateVersions = () => {
    // Clear version errors first
    errors.android_min_version = '';
    errors.android_current_version = '';
    errors.ios_min_version = '';
    errors.ios_current_version = '';

    // Validate Android format
    if (form.android_min_version && !isValidSemanticVersion(form.android_min_version)) {
        errors.android_min_version = 'Android min version must be in format: major.minor.patch (e.g., 1.0.0)';
    }

    if (form.android_current_version && !isValidSemanticVersion(form.android_current_version)) {
        errors.android_current_version = 'Android current version must be in format: major.minor.patch (e.g., 1.0.0)';
    }

    // Validate Android: min <= current
    if (form.android_min_version && form.android_current_version && !errors.android_min_version && !errors.android_current_version) {
        if (compareVersions(form.android_min_version, form.android_current_version) > 0) {
            errors.android_min_version = 'Android min version must be less than or equal to current version';
        }
    }

    // Validate iOS format
    if (form.ios_min_version && !isValidSemanticVersion(form.ios_min_version)) {
        errors.ios_min_version = 'iOS min version must be in format: major.minor.patch (e.g., 1.0.0)';
    }

    if (form.ios_current_version && !isValidSemanticVersion(form.ios_current_version)) {
        errors.ios_current_version = 'iOS current version must be in format: major.minor.patch (e.g., 1.0.0)';
    }

    // Validate iOS: min <= current
    if (form.ios_min_version && form.ios_current_version && !errors.ios_min_version && !errors.ios_current_version) {
        if (compareVersions(form.ios_min_version, form.ios_current_version) > 0) {
            errors.ios_min_version = 'iOS min version must be less than or equal to current version';
        }
    }
};

const submitForm = () => {
    // Clear previous errors
    Object.keys(errors).forEach(key => {
        errors[key] = '';
    });

    // Validate versions before submit
    validateVersions();

    // If there are validation errors, don't submit
    if (errors.android_min_version || errors.android_current_version || errors.ios_min_version || errors.ios_current_version) {
        return;
    }

    processing.value = true;

    const url = isEdit.value ? `/apps/${props.app?.id}` : '/apps';
    const method = isEdit.value ? 'put' : 'post';

    router[method](url, form, {
        onError: (formErrors: Record<string, string>) => {
            Object.keys(formErrors).forEach(key => {
                if (key in errors) {
                    errors[key] = formErrors[key];
                }
            });
        },
        onFinish: () => {
            processing.value = false;
        },
    });
};
</script>
