<template>
    <v-container fluid>
                <!-- Success Alert -->
                <v-alert
                    v-if="flash?.success"
                    type="success"
                    variant="tonal"
                    closable
                    class="mb-4"
                >
                    {{ flash.success }}
                </v-alert>

                <v-card>
                    <v-card-title class="d-flex align-center">
                        <v-icon class="mr-2">mdi-account-group</v-icon>
                        <span>Quản lý Clients</span>
                        <v-spacer></v-spacer>
                        <v-btn
                            color="primary"
                            @click="router.visit('/clients/create')"
                            prepend-icon="mdi-plus"
                        >
                            Thêm Client
                        </v-btn>
                    </v-card-title>

                    <v-divider></v-divider>

                    <!-- Filters -->
                    <v-card-text>
                        <v-row>
                            <v-col cols="12" md="6">
                                <v-text-field
                                    v-model="search"
                                    label="Tìm kiếm"
                                    prepend-inner-icon="mdi-magnify"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="handleSearch"
                                ></v-text-field>
                            </v-col>
                            <v-col cols="12" md="3">
                                <v-select
                                    v-model="statusFilter"
                                    :items="statusOptions"
                                    label="Trạng thái"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="handleFilter"
                                ></v-select>
                            </v-col>
                        </v-row>
                    </v-card-text>

                    <!-- Data Table -->
                    <v-data-table
                        :headers="headers"
                        :items="clients.data"
                        :loading="loading"
                        hide-default-footer
                    >
                        <template v-slot:item.status="{ item }">
                            <v-chip
                                :color="item.status === 'active' ? 'success' : 'error'"
                                size="small"
                            >
                                {{ item.status === 'active' ? 'Hoạt động' : 'Không hoạt động' }}
                            </v-chip>
                        </template>

                        <template v-slot:item.uuid="{ item }">
                            <code class="text-caption">{{ item.uuid }}</code>
                        </template>

                        <template v-slot:item.created_at="{ item }">
                            {{ formatDate(item.created_at) }}
                        </template>

                        <template v-slot:item.actions="{ item }">
                            <v-btn
                                icon
                                size="small"
                                variant="text"
                                @click="router.visit(`/clients/${item.id}/edit`)"
                            >
                                <v-icon>mdi-pencil</v-icon>
                                <v-tooltip activator="parent" location="top">Sửa</v-tooltip>
                            </v-btn>
                            <v-btn
                                icon
                                size="small"
                                variant="text"
                                color="error"
                                @click="confirmDelete(item)"
                            >
                                <v-icon>mdi-delete</v-icon>
                                <v-tooltip activator="parent" location="top">Xóa</v-tooltip>
                            </v-btn>
                        </template>
                    </v-data-table>

                    <!-- Pagination -->
                    <v-divider></v-divider>
                    <v-card-text class="d-flex justify-space-between align-center">
                        <div class="text-caption">
                            Hiển thị {{ clients.from }} - {{ clients.to }} / {{ clients.total }} bản ghi
                        </div>
                        <v-pagination
                            v-model="currentPage"
                            :length="clients.last_page"
                            :total-visible="7"
                            @update:model-value="handlePageChange"
                        ></v-pagination>
                    </v-card-text>
                </v-card>
            </v-container>

        <!-- Delete Confirmation Dialog -->
        <v-dialog v-model="deleteDialog" max-width="400">
            <v-card>
                <v-card-title class="text-h5">Xác nhận xóa</v-card-title>
                <v-card-text>
                    Bạn có chắc chắn muốn xóa client <strong>{{ selectedClient?.name }}</strong>?
                    Hành động này không thể hoàn tác.
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn variant="text" @click="deleteDialog = false">Hủy</v-btn>
                    <v-btn color="error" variant="flat" @click="deleteClient">Xóa</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
</template>

<script setup lang="ts">
import { router, usePage } from '@inertiajs/vue3';
import { computed, ref } from 'vue';
import DefaultLayout from '@/Layouts/DefaultLayout.vue';

interface Client {
    id: number;
    name: string;
    uuid: string;
    status: 'active' | 'inactive';
    created_at: string;
}

interface Props {
    clients: {
        data: Client[];
        current_page: number;
        last_page: number;
        from: number;
        to: number;
        total: number;
    };
    filters: {
        search: string | null;
        status: string | null;
    };
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
    flash?: {
        error?: string | null;
        success?: string | null;
    };
}

defineOptions({
    layout: DefaultLayout,
});

const props = defineProps<Props>();
const page = usePage<PageProps>();

const flash = computed(() => page.props.flash || null);

const search = ref(props.filters.search || '');
const statusFilter = ref(props.filters.status || null);
const currentPage = ref(props.clients.current_page);
const loading = ref(false);
const deleteDialog = ref(false);
const selectedClient = ref<Client | null>(null);

const headers = [
    { title: 'ID', key: 'id', sortable: false },
    { title: 'Tên', key: 'name', sortable: false },
    { title: 'UUID', key: 'uuid', sortable: false },
    { title: 'Trạng thái', key: 'status', sortable: false },
    { title: 'Ngày tạo', key: 'created_at', sortable: false },
    { title: 'Thao tác', key: 'actions', sortable: false, align: 'center' as const },
];

const statusOptions = [
    { title: 'Hoạt động', value: 'active' },
    { title: 'Không hoạt động', value: 'inactive' },
];

let searchTimeout: ReturnType<typeof setTimeout>;

const handleSearch = () => {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
        applyFilters();
    }, 500);
};

const handleFilter = () => {
    applyFilters();
};

const handlePageChange = (page: number) => {
    applyFilters(page);
};

const applyFilters = (page: number = 1) => {
    loading.value = true;
    router.get('/clients', {
        page,
        search: search.value,
        status: statusFilter.value,
    }, {
        preserveState: true,
        onFinish: () => {
            loading.value = false;
        },
    });
};

const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('vi-VN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
    });
};

const confirmDelete = (client: Client) => {
    selectedClient.value = client;
    deleteDialog.value = true;
};

const deleteClient = () => {
    if (!selectedClient.value) return;

    router.delete(`/clients/${selectedClient.value.id}`, {
        onSuccess: () => {
            deleteDialog.value = false;
            selectedClient.value = null;
        },
    });
};
</script>
