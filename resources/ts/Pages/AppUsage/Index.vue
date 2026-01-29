<template>
    <v-container fluid>
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
                <v-icon class="mr-2">mdi-chart-line</v-icon>
                Sử dụng App
            </v-card-title>
            <v-divider></v-divider>

            <v-tabs v-model="activeTab" grow>
                <v-tab value="devices">Thiết bị sử dụng app</v-tab>
                <v-tab value="logs">Lịch sử kết nối</v-tab>
            </v-tabs>

            <v-window v-model="activeTab">
                <!-- Tab: Thiết bị sử dụng app -->
                <v-window-item value="devices">
                    <v-card-text>
                        <v-row class="mb-4">
                            <v-col cols="12" md="4">
                                <v-text-field
                                    v-model="searchDevices"
                                    label="Tìm kiếm (username, device_id)"
                                    prepend-inner-icon="mdi-magnify"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="applyDevicesFilters"
                                ></v-text-field>
                            </v-col>
                            <v-col cols="12" md="3">
                                <v-select
                                    v-model="filterAppId"
                                    :items="appOptions"
                                    label="App"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="applyDevicesFilters"
                                ></v-select>
                            </v-col>
                            <v-col cols="12" md="3">
                                <v-select
                                    v-model="filterClientSystemId"
                                    :items="clientSystemOptions"
                                    label="Client System"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="applyDevicesFilters"
                                ></v-select>
                            </v-col>
                        </v-row>

                        <v-data-table
                            :headers="devicesHeaders"
                            :items="appUserDevices.data"
                            :loading="devicesLoading"
                            hide-default-footer
                        >
                            <template v-slot:item.app="{ item }">
                                <v-chip size="small" color="success" variant="tonal">
                                    {{ item.app?.name || '-' }}
                                </v-chip>
                            </template>
                            <template v-slot:item.client_system="{ item }">
                                <v-chip size="small" color="info" variant="tonal">
                                    {{ item.client_system?.name || '-' }}
                                </v-chip>
                            </template>
                            <template v-slot:item.user_device="{ item }">
                                <div>
                                    <code class="text-caption">{{ item.user_device?.device_id?.substring(0, 12) }}...</code>
                                    <div v-if="item.user_device?.device_info" class="text-caption text-medium-emphasis">
                                        {{ deviceInfoLabel(item.user_device.device_info) }}
                                    </div>
                                </div>
                            </template>
                            <template v-slot:item.app_version="{ item }">
                                <v-chip v-if="item.app_version" size="small" variant="tonal">
                                    {{ item.app_version }}
                                </v-chip>
                                <span v-else>-</span>
                            </template>
                            <template v-slot:item.last_connected_at="{ item }">
                                {{ item.last_connected_at ? formatDate(item.last_connected_at) : '-' }}
                            </template>
                            <template v-slot:item.created_at="{ item }">
                                {{ formatDate(item.created_at) }}
                            </template>
                        </v-data-table>

                        <v-divider></v-divider>
                        <v-card-text class="d-flex justify-space-between align-center">
                            <div class="text-caption">
                                Hiển thị {{ appUserDevices.from || 0 }} - {{ appUserDevices.to || 0 }} / {{ appUserDevices.total }} bản ghi
                            </div>
                            <v-pagination
                                v-model="devicesPage"
                                :length="appUserDevices.last_page"
                                :total-visible="5"
                                @update:model-value="handleDevicesPageChange"
                            ></v-pagination>
                        </v-card-text>
                    </v-card-text>
                </v-window-item>

                <!-- Tab: Lịch sử kết nối -->
                <v-window-item value="logs">
                    <v-card-text>
                        <v-row class="mb-4">
                            <v-col cols="12" md="3">
                                <v-select
                                    v-model="filterLogAppId"
                                    :items="appOptions"
                                    label="App"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="applyLogsFilters"
                                ></v-select>
                            </v-col>
                            <v-col cols="12" md="3">
                                <v-select
                                    v-model="filterLogClientSystemId"
                                    :items="clientSystemOptions"
                                    label="Client System"
                                    variant="outlined"
                                    density="compact"
                                    clearable
                                    @update:model-value="applyLogsFilters"
                                ></v-select>
                            </v-col>
                        </v-row>

                        <v-data-table
                            :headers="logsHeaders"
                            :items="connectionLogs.data"
                            :loading="logsLoading"
                            hide-default-footer
                        >
                            <template v-slot:item.app_user_device="{ item }">
                                <div v-if="item.app_user_device">
                                    <v-chip size="small" color="success" variant="tonal" class="mr-1">
                                        {{ item.app_user_device.app?.name }}
                                    </v-chip>
                                    <v-chip size="small" color="info" variant="tonal" class="mr-1">
                                        {{ item.app_user_device.client_system?.name }}
                                    </v-chip>
                                    <div class="text-caption mt-1">
                                        {{ item.app_user_device.username }} · <code>{{ item.app_user_device.user_device?.device_id?.substring(0, 8) }}...</code>
                                    </div>
                                </div>
                                <span v-else>-</span>
                            </template>
                            <template v-slot:item.app_version="{ item }">
                                <v-chip v-if="item.app_version" size="small" variant="tonal">
                                    {{ item.app_version }}
                                </v-chip>
                                <span v-else>-</span>
                            </template>
                            <template v-slot:item.ip_address="{ item }">
                                <code class="text-caption">{{ item.ip_address || '-' }}</code>
                            </template>
                            <template v-slot:item.created_at="{ item }">
                                {{ formatDate(item.created_at) }}
                            </template>
                        </v-data-table>

                        <v-divider></v-divider>
                        <v-card-text class="d-flex justify-space-between align-center">
                            <div class="text-caption">
                                Hiển thị {{ connectionLogs.from || 0 }} - {{ connectionLogs.to || 0 }} / {{ connectionLogs.total }} bản ghi
                            </div>
                            <v-pagination
                                v-model="logsPage"
                                :length="connectionLogs.last_page"
                                :total-visible="5"
                                @update:model-value="handleLogsPageChange"
                            ></v-pagination>
                        </v-card-text>
                    </v-card-text>
                </v-window-item>
            </v-window>
        </v-card>
    </v-container>
</template>

<script setup lang="ts">
import DefaultLayout from '@/Layouts/DefaultLayout.vue';
import { router, usePage } from '@inertiajs/vue3';
import { computed, ref } from 'vue';

interface UserDevice {
    id: number;
    device_id: string;
    device_info: Record<string, string> | null;
}

interface AppUserDevice {
    id: number;
    app_id: number;
    client_system_id: number;
    user_device_id: number;
    username: string;
    app_version: string | null;
    last_connected_at: string | null;
    created_at: string;
    app?: { id: number; name: string };
    client_system?: { id: number; name: string };
    user_device?: UserDevice;
}

interface AppConnectionLog {
    id: number;
    app_user_device_id: number;
    app_version: string | null;
    ip_address: string | null;
    created_at: string;
    app_user_device?: AppUserDevice & {
        app?: { id: number; name: string };
        client_system?: { id: number; name: string };
        user_device?: UserDevice;
    };
}

interface AppOption {
    id: number;
    name: string;
}

interface ClientSystemOption {
    id: number;
    name: string;
    client?: { id: number; name: string };
}

interface Props {
    appUserDevices: {
        data: AppUserDevice[];
        current_page: number;
        last_page: number;
        from: number;
        to: number;
        total: number;
    };
    connectionLogs: {
        data: AppConnectionLog[];
        current_page: number;
        last_page: number;
        from: number;
        to: number;
        total: number;
    };
    apps: AppOption[];
    clientSystems: ClientSystemOption[];
    filters: {
        app_id: string | null;
        client_system_id: string | null;
        search_devices: string | null;
        log_app_id: string | null;
        log_client_system_id: string | null;
    };
}

interface PageProps extends Record<string, any> {
    flash?: { success?: string | null };
}

defineOptions({
    layout: DefaultLayout,
});

const props = defineProps<Props>();
const page = usePage<PageProps>();

const flash = computed(() => page.props.flash || null);

const activeTab = ref('devices');
const devicesLoading = ref(false);
const logsLoading = ref(false);

const searchDevices = ref(props.filters.search_devices || '');
const filterAppId = ref(props.filters.app_id ? Number(props.filters.app_id) : null);
const filterClientSystemId = ref(props.filters.client_system_id ? Number(props.filters.client_system_id) : null);
const filterLogAppId = ref(props.filters.log_app_id ? Number(props.filters.log_app_id) : null);
const filterLogClientSystemId = ref(props.filters.log_client_system_id ? Number(props.filters.log_client_system_id) : null);
const devicesPage = ref(props.appUserDevices.current_page);
const logsPage = ref(props.connectionLogs.current_page);

const devicesHeaders = [
    { title: 'ID', key: 'id', sortable: false },
    { title: 'App', key: 'app', sortable: false },
    { title: 'Client System', key: 'client_system', sortable: false },
    { title: 'Username', key: 'username', sortable: false },
    { title: 'Thiết bị', key: 'user_device', sortable: false },
    { title: 'Phiên bản', key: 'app_version', sortable: false },
    { title: 'Kết nối lần cuối', key: 'last_connected_at', sortable: false },
    { title: 'Ngày tạo', key: 'created_at', sortable: false },
];

const logsHeaders = [
    { title: 'ID', key: 'id', sortable: false },
    { title: 'App · System · User', key: 'app_user_device', sortable: false },
    { title: 'Phiên bản', key: 'app_version', sortable: false },
    { title: 'IP', key: 'ip_address', sortable: false },
    { title: 'Thời gian', key: 'created_at', sortable: false },
];

const appOptions = computed(() =>
    props.apps.map(a => ({ title: a.name, value: a.id }))
);

const clientSystemOptions = computed(() =>
    props.clientSystems.map(cs => ({
        title: cs.client ? `${cs.name} (${cs.client.name})` : cs.name,
        value: cs.id,
    }))
);

const deviceInfoLabel = (info: Record<string, string> | null) => {
    if (!info) return '';
    const parts = [];
    if (info.platform) parts.push(info.platform);
    if (info.model) parts.push(info.model);
    if (info.app_version) parts.push(`v${info.app_version}`);
    return parts.join(' · ') || '-';
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

const buildQuery = () => ({
    app_id: filterAppId.value,
    client_system_id: filterClientSystemId.value,
    search_devices: searchDevices.value,
    devices_page: devicesPage.value,
    log_app_id: filterLogAppId.value,
    log_client_system_id: filterLogClientSystemId.value,
    logs_page: logsPage.value,
});

let devicesSearchTimeout: ReturnType<typeof setTimeout>;
const applyDevicesFilters = () => {
    clearTimeout(devicesSearchTimeout);
    devicesSearchTimeout = setTimeout(() => {
        devicesPage.value = 1;
        devicesLoading.value = true;
        router.get('/app-usage', buildQuery(), {
            preserveState: true,
            onFinish: () => { devicesLoading.value = false; },
        });
    }, 400);
};

const handleDevicesPageChange = (p: number) => {
    devicesPage.value = p;
    devicesLoading.value = true;
    router.get('/app-usage', buildQuery(), {
        preserveState: true,
        onFinish: () => { devicesLoading.value = false; },
    });
};

const applyLogsFilters = () => {
    logsPage.value = 1;
    logsLoading.value = true;
    router.get('/app-usage', buildQuery(), {
        preserveState: true,
        onFinish: () => { logsLoading.value = false; },
    });
};

const handleLogsPageChange = (p: number) => {
    logsPage.value = p;
    logsLoading.value = true;
    router.get('/app-usage', { ...buildQuery(), logs_page: p }, {
        preserveState: true,
        onFinish: () => { logsLoading.value = false; },
    });
};
</script>
