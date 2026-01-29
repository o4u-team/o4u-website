<template>
    <v-container>
        <v-row justify="center">
            <v-col cols="12" md="8" lg="6">
                <v-card>
                    <v-card-title class="d-flex align-center">
                        <v-btn
                            icon
                            variant="text"
                            @click="router.visit('/client-systems')"
                            class="mr-2"
                        >
                            <v-icon>mdi-arrow-left</v-icon>
                            <v-tooltip activator="parent" location="bottom">Quay lại</v-tooltip>
                        </v-btn>
                        <v-icon class="mr-2">
                            {{ isEdit ? 'mdi-pencil' : 'mdi-plus' }}
                        </v-icon>
                        {{ isEdit ? 'Chỉnh sửa Client System' : 'Tạo Client System mới' }}
                    </v-card-title>

                    <v-divider></v-divider>

                    <v-card-text>
                        <v-form ref="formRef" @submit.prevent="submitForm">
                            <v-text-field
                                v-model="form.name"
                                label="Tên System *"
                                prepend-inner-icon="mdi-server"
                                variant="outlined"
                                :error-messages="errors.name"
                                required
                            ></v-text-field>

                            <v-select
                                v-model="form.client_id"
                                :items="clientOptions"
                                label="Client *"
                                prepend-inner-icon="mdi-account-group"
                                variant="outlined"
                                :error-messages="errors.client_id"
                                required
                            ></v-select>

                            <v-text-field
                                v-model="form.endpoint"
                                label="Endpoint *"
                                prepend-inner-icon="mdi-link"
                                variant="outlined"
                                :error-messages="errors.endpoint"
                                placeholder="https://api.example.com"
                                required
                            ></v-text-field>

                            <v-text-field
                                v-model="form.db_name"
                                label="Database Name *"
                                prepend-inner-icon="mdi-database"
                                variant="outlined"
                                :error-messages="errors.db_name"
                                placeholder="database_name"
                                required
                            ></v-text-field>

                            <v-select
                                v-model="form.status"
                                :items="statusOptions"
                                label="Trạng thái *"
                                prepend-inner-icon="mdi-check-circle"
                                variant="outlined"
                                :error-messages="errors.status"
                                required
                            ></v-select>

                            <v-alert
                                v-if="isEdit && clientSystem?.uuid"
                                type="info"
                                variant="tonal"
                                class="mb-4"
                            >
                                <div class="text-subtitle-2 mb-1">UUID (không thể thay đổi)</div>
                                <code class="text-body-2">{{ clientSystem.uuid }}</code>
                            </v-alert>

                            <div class="text-caption text-medium-emphasis mb-4">
                                * Trường bắt buộc
                            </div>
                        </v-form>
                    </v-card-text>

                    <v-divider></v-divider>

                    <v-card-actions class="pa-4">
                        <v-btn
                            variant="text"
                            @click="router.visit('/client-systems')"
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

interface Client {
    id: number;
    name: string;
}

interface ClientSystem {
    id: number;
    name: string;
    uuid: string;
    client_id: number;
    endpoint: string;
    db_name: string;
    status: 'active' | 'inactive' | 'expired';
}

interface Props {
    clientSystem?: ClientSystem | null;
    clients: Client[];
}

defineOptions({
    layout: DefaultLayout,
});

const props = defineProps<Props>();

const isEdit = computed(() => !!props.clientSystem);

const formRef = ref();
const processing = ref(false);

const form = reactive({
    name: props.clientSystem?.name || '',
    client_id: props.clientSystem?.client_id || null as number | null,
    endpoint: props.clientSystem?.endpoint || '',
    db_name: props.clientSystem?.db_name || '',
    status: props.clientSystem?.status || 'active',
});

const errors = reactive<Record<string, string>>({
    name: '',
    client_id: '',
    endpoint: '',
    db_name: '',
    status: '',
});

const statusOptions = [
    { title: 'Hoạt động', value: 'active' },
    { title: 'Không hoạt động', value: 'inactive' },
    { title: 'Hết hạn', value: 'expired' },
];

const clientOptions = computed(() =>
    props.clients.map(client => ({
        title: client.name,
        value: client.id,
    }))
);

const submitForm = () => {
    // Clear previous errors
    Object.keys(errors).forEach(key => {
        errors[key] = '';
    });

    processing.value = true;

    const url = isEdit.value ? `/client-systems/${props.clientSystem?.id}` : '/client-systems';
    const method = isEdit.value ? 'put' : 'post';

    router[method](url, form, {
        onError: (formErrors) => {
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
