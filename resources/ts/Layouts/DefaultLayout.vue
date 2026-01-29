<template>
    <v-app>
        <v-app-bar color="primary" prominent>
            <v-app-bar-title style="cursor: pointer" @click="router.visit('/')">
                O4U Dashboard
            </v-app-bar-title>

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
            <slot />
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { router, usePage } from '@inertiajs/vue3';

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

const page = usePage<PageProps>();

const user = computed(() => page.props.auth?.user || null);

const logout = () => {
    router.post('/logout');
};
</script>
