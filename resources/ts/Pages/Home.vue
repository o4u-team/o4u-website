<template>
    <v-app>
        <v-app-bar color="primary" prominent>
            <v-app-bar-title>O4U Dashboard</v-app-bar-title>

            <v-spacer></v-spacer>

            <v-menu>
                <template v-slot:activator="{ props }">
                    <v-btn icon v-bind="props">
                        <v-avatar v-if="user?.avatar" :image="user.avatar"></v-avatar>
                        <v-icon v-else>mdi-account-circle</v-icon>
                    </v-btn>
                </template>

                <v-list>
                    <v-list-item>
                        <v-list-item-title>{{ user?.name }}</v-list-item-title>
                        <v-list-item-subtitle>{{ user?.email }}</v-list-item-subtitle>
                    </v-list-item>
                    <v-divider></v-divider>
                    <v-list-item @click="logout">
                        <template v-slot:prepend>
                            <v-icon>mdi-logout</v-icon>
                        </template>
                        <v-list-item-title>Đăng xuất</v-list-item-title>
                    </v-list-item>
                </v-list>
            </v-menu>
        </v-app-bar>

        <v-main>
            <v-container>
                <v-row>
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
                </v-row>
            </v-container>
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import { router, usePage } from '@inertiajs/vue3';
import { computed } from 'vue';

const page = usePage();

const user = computed(() => page.props.auth?.user || null);

const logout = () => {
    router.post('/logout');
};
</script>
