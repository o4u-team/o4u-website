<template>
    <v-container>
        <v-row>
            <!-- Welcome Card -->
            <v-col cols="12">
                <v-card>
                    <v-card-title>
                        <v-icon class="mr-2">mdi-home</v-icon>
                        Chào mừng, {{ user?.name }}!
                    </v-card-title>
                    <v-card-text>
                        <p class="text-h6 mb-4">
                            Bạn đã đăng nhập thành công với Google OAuth
                        </p>
                        <v-chip color="success" class="mb-2">
                            <v-icon start>mdi-check-circle</v-icon>
                            Đã xác thực
                        </v-chip>
                    </v-card-text>
                </v-card>
            </v-col>

            <!-- Section 1: Quản lý App -->
            <v-col cols="12">
                <div class="text-h6 mb-3 d-flex align-center">
                    <v-icon class="mr-2" color="success">mdi-application</v-icon>
                    Quản lý Ứng dụng
                </div>
                <v-row>
                    <v-col cols="12" md="6" lg="4">
                        <v-card hover @click="router.visit('/apps')">
                            <v-card-title class="d-flex align-center">
                                <v-icon class="mr-2" color="success" size="large">mdi-application</v-icon>
                                <div>
                                    <div>Quản lý Apps</div>
                                    <div class="text-caption text-medium-emphasis">
                                        Quản lý các ứng dụng và phiên bản
                                    </div>
                                </div>
                                <v-spacer></v-spacer>
                                <v-icon>mdi-chevron-right</v-icon>
                            </v-card-title>
                        </v-card>
                    </v-col>
                    <v-col cols="12" md="6" lg="4">
                        <v-card hover @click="router.visit('/app-usage')">
                            <v-card-title class="d-flex align-center">
                                <v-icon class="mr-2" color="warning" size="large">mdi-chart-line</v-icon>
                                <div>
                                    <div>Sử dụng App</div>
                                    <div class="text-caption text-medium-emphasis">
                                        Thiết bị và lịch sử kết nối
                                    </div>
                                </div>
                                <v-spacer></v-spacer>
                                <v-icon>mdi-chevron-right</v-icon>
                            </v-card-title>
                        </v-card>
                    </v-col>
                </v-row>
            </v-col>

            <!-- Section 2: Quản lý Client -->
            <v-col cols="12">
                <div class="text-h6 mb-3 d-flex align-center">
                    <v-icon class="mr-2" color="primary">mdi-account-group</v-icon>
                    Quản lý Khách hàng
                </div>
                <v-row>
                    <v-col cols="12" md="6" lg="4">
                        <v-card hover @click="router.visit('/clients')">
                            <v-card-title class="d-flex align-center">
                                <v-icon class="mr-2" color="primary" size="large">mdi-account-group</v-icon>
                                <div>
                                    <div>Quản lý Clients</div>
                                    <div class="text-caption text-medium-emphasis">
                                        Thêm, sửa, xóa clients
                                    </div>
                                </div>
                                <v-spacer></v-spacer>
                                <v-icon>mdi-chevron-right</v-icon>
                            </v-card-title>
                        </v-card>
                    </v-col>

                    <v-col cols="12" md="6" lg="4">
                        <v-card hover @click="router.visit('/client-systems')">
                            <v-card-title class="d-flex align-center">
                                <v-icon class="mr-2" color="info" size="large">mdi-server</v-icon>
                                <div>
                                    <div>Quản lý Client Systems</div>
                                    <div class="text-caption text-medium-emphasis">
                                        Quản lý hệ thống của khách hàng
                                    </div>
                                </div>
                                <v-spacer></v-spacer>
                                <v-icon>mdi-chevron-right</v-icon>
                            </v-card-title>
                        </v-card>
                    </v-col>
                </v-row>
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup lang="ts">
import DefaultLayout from '@/Layouts/DefaultLayout.vue';
import { router, usePage } from '@inertiajs/vue3';
import { computed } from 'vue';

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

const page = usePage<PageProps>();

const user = computed(() => page.props.auth?.user || null);
</script>
