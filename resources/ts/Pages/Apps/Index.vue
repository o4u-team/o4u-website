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
                <v-icon class="mr-2">mdi-application</v-icon>
                <span>Quản lý Apps</span>
                <v-spacer></v-spacer>
                <v-btn
                    color="primary"
                    @click="router.visit('/apps/create')"
                    prepend-icon="mdi-plus"
                >
                    Thêm App
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
                :items="apps.data"
                :loading="loading"
                hide-default-footer
            >
                <template v-slot:item.status="{ item }">
                    <v-chip
                        :color="getStatusColor(item.status)"
                        size="small"
                    >
                        {{ getStatusLabel(item.status) }}
                    </v-chip>
                </template>

                <template v-slot:item.uuid="{ item }">
                    <code class="text-caption">{{ item.uuid }}</code>
                </template>

                <template v-slot:item.versions="{ item }">
                    <div class="text-caption">
                        <div v-if="item.android_min_version || item.android_current_version" class="mb-1">
                            <v-icon size="x-small" color="success" class="mr-1">mdi-android</v-icon>
                            <span v-if="item.android_min_version">Min: <strong>{{ item.android_min_version }}</strong></span>
                            <span v-if="item.android_min_version && item.android_current_version"> / </span>
                            <span v-if="item.android_current_version">Current: <strong>{{ item.android_current_version }}</strong></span>
                        </div>
                        <div v-if="item.ios_min_version || item.ios_current_version">
                            <v-icon size="x-small" class="mr-1">mdi-apple</v-icon>
                            <span v-if="item.ios_min_version">Min: <strong>{{ item.ios_min_version }}</strong></span>
                            <span v-if="item.ios_min_version && item.ios_current_version"> / </span>
                            <span v-if="item.ios_current_version">Current: <strong>{{ item.ios_current_version }}</strong></span>
                        </div>
                        <div v-if="!item.android_min_version && !item.android_current_version && !item.ios_min_version && !item.ios_current_version" class="text-medium-emphasis">-</div>
                    </div>
                </template>

                <template v-slot:item.stores="{ item }">
                    <div class="d-flex gap-1">
                        <v-btn
                            v-if="item.android_store_url"
                            icon
                            size="x-small"
                            variant="text"
                            :href="item.android_store_url"
                            target="_blank"
                        >
                            <v-icon color="success">mdi-android</v-icon>
                            <v-tooltip activator="parent">Android Store</v-tooltip>
                        </v-btn>
                        <v-btn
                            v-if="item.apple_store_url"
                            icon
                            size="x-small"
                            variant="text"
                            :href="item.apple_store_url"
                            target="_blank"
                        >
                            <v-icon>mdi-apple</v-icon>
                            <v-tooltip activator="parent">Apple Store</v-tooltip>
                        </v-btn>
                        <span v-if="!item.android_store_url && !item.apple_store_url" class="text-caption text-medium-emphasis">-</span>
                    </div>
                </template>

                <template v-slot:item.created_at="{ item }">
                    {{ formatDate(item.created_at) }}
                </template>

                <template v-slot:item.actions="{ item }">
                    <v-btn
                        icon
                        size="small"
                        variant="text"
                        @click="router.visit(`/apps/${item.id}/edit`)"
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
                    Hiển thị {{ apps.from }} - {{ apps.to }} / {{ apps.total }} bản ghi
                </div>
                <v-pagination
                    v-model="currentPage"
                    :length="apps.last_page"
                    :total-visible="7"
                    @update:model-value="handlePageChange"
                ></v-pagination>
            </v-card-text>
        </v-card>

        <!-- Delete Confirmation Dialog -->
        <v-dialog v-model="deleteDialog" max-width="400">
            <v-card>
                <v-card-title class="text-h5">Xác nhận xóa</v-card-title>
                <v-card-text>
                    Bạn có chắc chắn muốn xóa app <strong>{{ selectedApp?.name }}</strong>?
                    Hành động này không thể hoàn tác.
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn variant="text" @click="deleteDialog = false">Hủy</v-btn>
                    <v-btn color="error" variant="flat" @click="deleteApp">Xóa</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-container>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { router, usePage } from '@inertiajs/vue3';
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
    created_at: string;
}

interface Props {
    apps: {
        data: AppItem[];
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
const currentPage = ref(props.apps.current_page);
const loading = ref(false);
const deleteDialog = ref(false);
const selectedApp = ref<AppItem | null>(null);

const headers = [
    { title: 'ID', key: 'id', sortable: false },
    { title: 'Tên', key: 'name', sortable: false },
    { title: 'UUID', key: 'uuid', sortable: false },
    { title: 'Versions', key: 'versions', sortable: false },
    { title: 'Stores', key: 'stores', sortable: false },
    { title: 'Trạng thái', key: 'status', sortable: false },
    { title: 'Ngày tạo', key: 'created_at', sortable: false },
    { title: 'Thao tác', key: 'actions', sortable: false, align: 'center' as const },
];

const statusOptions = [
    { title: 'Hoạt động', value: 'active' },
    { title: 'Bảo trì', value: 'maintenance' },
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
    router.get('/apps', {
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

const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
        active: 'success',
        maintenance: 'warning',
        inactive: 'error',
    };
    return colors[status] || 'default';
};

const getStatusLabel = (status: string) => {
    const labels: Record<string, string> = {
        active: 'Hoạt động',
        maintenance: 'Bảo trì',
        inactive: 'Không hoạt động',
    };
    return labels[status] || status;
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

const confirmDelete = (app: AppItem) => {
    selectedApp.value = app;
    deleteDialog.value = true;
};

const deleteApp = () => {
    if (!selectedApp.value) return;

    router.delete(`/apps/${selectedApp.value.id}`, {
        onSuccess: () => {
            deleteDialog.value = false;
            selectedApp.value = null;
        },
    });
};
</script>
