<template>
    <h1 class="text-h4 mb-6">Thiết bị & lịch sử kết nối</h1>
    <v-alert v-if="error" type="error" class="mb-5">{{ error }}</v-alert
    ><v-tabs v-model="tab"
        ><v-tab value="devices">Thiết bị</v-tab
        ><v-tab value="logs">Lịch sử</v-tab></v-tabs
    >
    <form class="admin-toolbar mt-5" @submit.prevent="search">
        <v-select
            v-model="appId"
            label="Ứng dụng"
            :items="data?.apps ?? []"
            item-title="name"
            item-value="id"
            clearable
            hide-details
        /><v-select
            v-model="systemId"
            label="Hệ thống"
            :items="data?.clientSystems ?? []"
            item-title="name"
            item-value="id"
            clearable
            hide-details
        /><v-text-field
            v-if="tab === 'devices'"
            v-model="term"
            label="Username / Device ID"
            hide-details
        /><v-btn type="submit" :loading="busy">Lọc</v-btn>
    </form>
    <v-progress-linear v-if="busy" indeterminate />
    <v-card v-if="data" variant="outlined"
        ><div class="table-scroll">
            <v-table v-if="tab === 'devices'"
                ><thead>
                    <tr>
                        <th>Người dùng</th>
                        <th>Ứng dụng</th>
                        <th>Hệ thống</th>
                        <th>Thiết bị</th>
                        <th>Phiên bản</th>
                        <th>Kết nối gần nhất</th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="device in data.appUserDevices.data"
                        :key="device.id"
                    >
                        <td>{{ device.username }}</td>
                        <td>{{ device.app.name }}</td>
                        <td>{{ device.client_system.name }}</td>
                        <td>
                            {{ device.user_device.device_id }}
                            <details>
                                <summary>Thông tin thiết bị</summary>
                                <pre>{{
                                    JSON.stringify(
                                        device.user_device.device_info,
                                        null,
                                        2,
                                    )
                                }}</pre>
                            </details>
                        </td>
                        <td>{{ device.app_version }}</td>
                        <td>{{ device.last_connected_at }}</td>
                    </tr>
                </tbody></v-table
            ><v-table v-else
                ><thead>
                    <tr>
                        <th>Người dùng</th>
                        <th>Ứng dụng / Hệ thống</th>
                        <th>Thiết bị</th>
                        <th>Phiên bản</th>
                        <th>IP</th>
                        <th>Thời gian</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="log in data.connectionLogs.data" :key="log.id">
                        <td>{{ log.app_user_device.username }}</td>
                        <td>
                            {{ log.app_user_device.app.name }} /
                            {{ log.app_user_device.client_system.name }}
                        </td>
                        <td>{{ log.app_user_device.user_device.device_id }}</td>
                        <td>{{ log.app_version }}</td>
                        <td>{{ log.ip_address }}</td>
                        <td>{{ log.created_at }}</td>
                    </tr>
                </tbody></v-table
            >
        </div>
        <p v-if="total === 0" class="empty-state">Chưa có dữ liệu.</p></v-card
    >
    <div class="admin-toolbar mt-5">
        <span>{{ total }} bản ghi</span
        ><v-pagination
            v-model="page"
            :length="lastPage"
            :disabled="busy"
            @update:model-value="load"
        />
    </div>
</template>
<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';
import { useO4u } from '../../app/runtime';
import type { Usage } from '../../domain/entities';
import { errorMessage } from '../composables/resources';
const data = ref<Usage | null>(null);
const o4u = useO4u();
const busy = ref(false);
const error = ref('');
const tab = ref('devices');
const appId = ref<number | null>(null);
const systemId = ref<number | null>(null);
const term = ref('');
const page = ref(1);
const current = computed(() =>
    tab.value === 'devices'
        ? data.value?.appUserDevices
        : data.value?.connectionLogs,
);
const total = computed(() => current.value?.total ?? 0);
const lastPage = computed(() => current.value?.last_page ?? 1);
async function load() {
    busy.value = true;
    error.value = '';
    try {
        const prefix = tab.value === 'devices' ? '' : 'log_';
        data.value = await o4u.repository.usage({
            [`${prefix}app_id`]: appId.value ?? '',
            [`${prefix}client_system_id`]: systemId.value ?? '',
            search_devices: term.value,
            devices_page: page.value,
            logs_page: page.value,
        });
    } catch (e) {
        error.value = errorMessage(e);
    } finally {
        busy.value = false;
    }
}
function search() {
    page.value = 1;
    void load();
}
watch(tab, search);
onMounted(load);
</script>
