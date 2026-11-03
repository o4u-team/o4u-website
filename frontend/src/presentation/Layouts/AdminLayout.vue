<template>
    <v-app>
        <v-app-bar color="primary"
            ><v-app-bar-title
                ><RouterLink
                    to="/dashboard"
                    class="text-white text-decoration-none"
                    >O4U Dashboard</RouterLink
                ></v-app-bar-title
            ><v-spacer /><span class="mr-4">{{ user?.name }}</span
            ><v-btn :loading="busy" @click="logout">Đăng xuất</v-btn></v-app-bar
        >
        <v-main
            ><nav class="admin-container admin-toolbar">
                <v-btn
                    v-for="item in navigation"
                    :key="item.path"
                    :to="item.path"
                    variant="text"
                    >{{ item.label }}</v-btn
                ><v-btn to="/" variant="text">Trang chủ</v-btn>
            </nav>
            <div class="admin-container">
                <p v-if="error" role="alert" class="error-text">{{ error }}</p>
                <RouterView :key="route.fullPath" /></div
        ></v-main>
    </v-app>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useSession } from '../composables/session';
const session = useSession();
const { user } = session;
const route = useRoute();
const router = useRouter();
const busy = ref(false);
const error = ref('');
const navigation = [
    { path: '/clients', label: 'Khách hàng' },
    { path: '/client-systems', label: 'Hệ thống khách hàng' },
    { path: '/apps', label: 'Ứng dụng' },
    { path: '/app-usage', label: 'Thiết bị & lịch sử' },
];
async function logout() {
    busy.value = true;
    try {
        await session.logout();
        await router.push('/login');
    } catch {
        error.value = 'Không thể đăng xuất. Vui lòng thử lại.';
    } finally {
        busy.value = false;
    }
}
</script>
