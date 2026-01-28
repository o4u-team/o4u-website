<template>
    <v-app>
        <v-main>
            <v-container fluid class="fill-height">
                <v-row align="center" justify="center">
                    <v-col cols="12" sm="8" md="4">
                        <v-card elevation="12" rounded="lg">
                            <v-card-title class="text-h5 text-center pa-6">
                                <div class="d-flex flex-column align-center">
                                    <v-icon size="64" color="primary" class="mb-4">
                                        mdi-account-circle
                                    </v-icon>
                                    <div>Đăng nhập</div>
                                </div>
                            </v-card-title>

                            <v-card-text class="pa-6">
                                <v-alert
                                    v-if="error"
                                    type="error"
                                    variant="tonal"
                                    closable
                                    class="mb-4"
                                >
                                    {{ error }}
                                </v-alert>

                                <div class="text-center mb-4">
                                    <p class="text-body-1 mb-2">
                                        Đăng nhập bằng tài khoản Google của bạn
                                    </p>
                                </div>

                                <v-btn
                                    :href="route('auth.google')"
                                    color="white"
                                    variant="elevated"
                                    size="large"
                                    block
                                    class="text-none"
                                    prepend-icon="mdi-google"
                                >
                                    <span class="text-body-1">Đăng nhập với Google</span>
                                </v-btn>
                            </v-card-text>
                        </v-card>
                    </v-col>
                </v-row>
            </v-container>
        </v-main>
    </v-app>
</template>

<script setup lang="ts">
import { usePage } from '@inertiajs/vue3';
import { computed } from 'vue';

const page = usePage();

// Get error from flash session
const error = computed(() => {
    return page.props.flash?.error || null;
});

// Helper function to generate route URL
const route = (name: string) => {
    const routes: Record<string, string> = {
        'auth.google': '/auth/google',
    };
    return routes[name] || '/';
};
</script>

<style scoped>
.fill-height {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.v-card {
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3) !important;
}
</style>
