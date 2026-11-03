<template>
    <v-app
        ><v-main
            ><v-container class="d-flex justify-center py-16"
                ><v-card width="480" class="pa-8"
                    ><h1 class="text-h4 mb-5">Đăng nhập O4U</h1>
                    <p class="mb-5">
                        Dùng tài khoản Google đã được cấp quyền quản trị.
                    </p>
                    <v-alert v-if="route.query.error" type="error" class="mb-5"
                        >Không thể đăng nhập. Kiểm tra kết nối và tài khoản được
                        cấp quyền.</v-alert
                    ><v-btn
                        :href="googleUrl"
                        color="primary"
                        block
                        prepend-icon="mdi-google"
                        >Đăng nhập với Google</v-btn
                    ><v-btn to="/" variant="text" class="mt-4"
                        >Về trang chủ</v-btn
                    ></v-card
                ></v-container
            ></v-main
        ></v-app
    >
</template>
<script setup lang="ts">
import { useRoute } from 'vue-router';
const route = useRoute();
const googleUrl = `${import.meta.env.VITE_API_BASE_URL ?? ''}/auth/google`;
</script>
