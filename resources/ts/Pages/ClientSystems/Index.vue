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
                <v-icon class="mr-2">mdi-server</v-icon>
                <span>Quản lý Client Systems</span>
                <v-spacer></v-spacer>
                <v-btn
                    color="primary"
                    @click="router.visit('/client-systems/create')"
                    prepend-icon="mdi-plus"
                >
                    Thêm System
                </v-btn>
            </v-card-title>

            <v-divider></v-divider>

            <!-- Filters -->
            <v-card-text>
                <v-row>
                    <v-col cols="12" md="4">
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
                            v-model="clientFilter"
                            :items="clientOptions"
                            label="Client"
                            variant="outlined"
                            density="compact"
                            clearable
                            @update:model-value="handleFilter"
                        ></v-select>
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
                :items="clientSystems.data"
                :loading="loading"
                hide-default-footer
            >
                <template v-slot:item.client="{ item }">
                    <v-chip size="small" color="info" variant="tonal">
                        {{ item.client?.name || '-' }}
                    </v-chip>
                </template>

                <template v-slot:item.status="{ item }">
                    <v-chip
                        :color="getStatusColor(item.status)"
                        size="small"
                    >
                        {{ getStatusLabel(item.status) }}
                    </v-chip>
                </template>

                <template v-slot:item.uuid="{ item }">
                    <code class="text-caption">{{ item.uuid?.substring(0, 8) }}...</code>
                    <v-tooltip activator="parent" location="top">{{ item.uuid }}</v-tooltip>
                </template>

                <template v-slot:item.endpoint="{ item }">
                    <code class="text-caption">{{ item.endpoint }}</code>
                </template>

                <template v-slot:item.db_name="{ item }">
                    <code class="text-caption">{{ item.db_name }}</code>
                </template>

                <template v-slot:item.created_at="{ item }">
                    {{ formatDate(item.created_at) }}
                </template>

                <template v-slot:item.actions="{ item }">
                    <v-btn
                        icon
                        size="small"
                        variant="text"
                        color="primary"
                        @click="router.visit(`/client-systems/${item.id}/app-info`)"
                    >
                        <v-icon>mdi-apps</v-icon>
                        <v-tooltip activator="parent" location="top">Cấu hình App Info</v-tooltip>
                    </v-btn>
                    <v-btn
                        icon
                        size="small"
                        variant="text"
                        @click="router.visit(`/client-systems/${item.id}/edit`)"
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
                    Hiển thị {{ clientSystems.from || 0 }} - {{ clientSystems.to || 0 }} / {{ clientSystems.total }} bản ghi
                </div>
                <v-pagination
                    v-model="currentPage"
                    :length="clientSystems.last_page"
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
                Bạn có chắc chắn muốn xóa system <strong>{{ selectedSystem?.name }}</strong>?
                Hành động này không thể hoàn tác.
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn variant="text" @click="deleteDialog = false">Hủy</v-btn>
                <v-btn color="error" variant="flat" @click="deleteSystem">Xóa</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>

</template>

<script setup lang="ts">
import DefaultLayout from '@/Layouts/DefaultLayout.vue';
import { router, usePage } from '@inertiajs/vue3';
import { computed, ref } from 'vue';

interface Client {
    id: number;
    name: string;
}

interface ClientSystem {
    id: number;
    name: string;
    uuid: string;
    client_id: number;
    client?: Client;
    endpoint: string;
    db_name: string;
    status: 'active' | 'inactive' | 'expired';
    created_at: string;
}

interface Props {
    clientSystems: {
        data: ClientSystem[];
        current_page: number;
        last_page: number;
        from: number;
        to: number;
        total: number;
    };
    clients: Client[];
    filters: {
        search: string | null;
        status: string | null;
        client_id: string | null;
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
const clientFilter = ref(props.filters.client_id ? Number(props.filters.client_id) : null);
const currentPage = ref(props.clientSystems.current_page);
const loading = ref(false);
const deleteDialog = ref(false);
const selectedSystem = ref<ClientSystem | null>(null);

const headers = [
    { title: 'ID', key: 'id', sortable: false },
    { title: 'Tên', key: 'name', sortable: false },
    { title: 'UUID', key: 'uuid', sortable: false },
    { title: 'Client', key: 'client', sortable: false },
    { title: 'Endpoint', key: 'endpoint', sortable: false },
    { title: 'DB Name', key: 'db_name', sortable: false },
    { title: 'Trạng thái', key: 'status', sortable: false },
    { title: 'Ngày tạo', key: 'created_at', sortable: false },
    { title: 'Thao tác', key: 'actions', sortable: false, align: 'center' as const },
];

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

const getStatusColor = (status: string) => {
    switch (status) {
        case 'active': return 'success';
        case 'inactive': return 'error';
        case 'expired': return 'warning';
        default: return 'grey';
    }
};

const getStatusLabel = (status: string) => {
    switch (status) {
        case 'active': return 'Hoạt động';
        case 'inactive': return 'Không hoạt động';
        case 'expired': return 'Hết hạn';
        default: return status;
    }
};

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
    router.get('/client-systems', {
        page,
        search: search.value,
        status: statusFilter.value,
        client_id: clientFilter.value,
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

const confirmDelete = (system: ClientSystem) => {
    selectedSystem.value = system;
    deleteDialog.value = true;
};

const deleteSystem = () => {
    if (!selectedSystem.value) return;

    router.delete(`/client-systems/${selectedSystem.value.id}`, {
        onSuccess: () => {
            deleteDialog.value = false;
            selectedSystem.value = null;
        },
    });
};

</script>
