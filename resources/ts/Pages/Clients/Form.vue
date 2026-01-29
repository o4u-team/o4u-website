<template>
    <v-container>
                <v-row justify="center">
                    <v-col cols="12" md="8" lg="6">
                        <v-card>
                            <v-card-title class="d-flex align-center">
                                <v-btn
                                    icon
                                    variant="text"
                                    @click="router.visit('/clients')"
                                    class="mr-2"
                                >
                                    <v-icon>mdi-arrow-left</v-icon>
                                    <v-tooltip activator="parent" location="bottom">Quay lại</v-tooltip>
                                </v-btn>
                                <v-icon class="mr-2">
                                    {{ isEdit ? 'mdi-pencil' : 'mdi-plus' }}
                                </v-icon>
                                {{ isEdit ? 'Chỉnh sửa thông tin Client' : 'Tạo Client mới' }}
                            </v-card-title>

                            <v-divider></v-divider>

                            <v-card-text>
                                <v-form ref="formRef" @submit.prevent="submitForm">
                                    <v-text-field
                                        v-model="form.name"
                                        label="Tên Client *"
                                        prepend-inner-icon="mdi-account"
                                        variant="outlined"
                                        :error-messages="errors.name"
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
                                        v-if="isEdit && client?.uuid"
                                        type="info"
                                        variant="tonal"
                                        class="mb-4"
                                    >
                                        <div class="text-subtitle-2 mb-1">UUID (không thể thay đổi)</div>
                                        <code class="text-body-2">{{ client.uuid }}</code>
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
                                    @click="router.visit('/clients')"
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
import { router, usePage } from '@inertiajs/vue3';
import DefaultLayout from '@/Layouts/DefaultLayout.vue';

interface Client {
    id: number;
    name: string;
    uuid: string;
    status: 'active' | 'inactive';
}

interface Props {
    client?: Client | null;
}

interface PageProps extends Record<string, any> {
    auth?: {
        user?: {
            id: number;
            name: string;
            email: string;
            avatar: string | null;
        } | null;
    };
}

defineOptions({
    layout: DefaultLayout,
});

const props = defineProps<Props>();
const page = usePage<PageProps>();

const isEdit = computed(() => !!props.client);

const formRef = ref();
const processing = ref(false);

const form = reactive({
    name: props.client?.name || '',
    status: props.client?.status || 'active',
});

const errors = reactive<Record<string, string>>({
    name: '',
    status: '',
});

const statusOptions = [
    { title: 'Hoạt động', value: 'active' },
    { title: 'Không hoạt động', value: 'inactive' },
];

const submitForm = () => {
    // Clear previous errors
    Object.keys(errors).forEach(key => {
        errors[key] = '';
    });

    processing.value = true;

    const url = isEdit.value ? `/clients/${props.client?.id}` : '/clients';
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
