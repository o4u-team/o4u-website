<template>
    <v-app>
        <v-app-bar
            :elevation="scrolled ? 4 : 0"
            :class="['landing-app-bar', { 'landing-app-bar--scrolled': scrolled }]"
            height="72"
        >
            <v-container class="d-flex align-center py-0">
                <div class="d-flex align-center ga-3 landing-brand" @click="router.visit('/')">
                    <v-avatar color="primary" size="44" rounded="lg">
                        <span class="landing-logo-mark font-weight-bold text-white">O4U</span>
                    </v-avatar>
                    <div>
                        <div class="text-subtitle-1 font-weight-bold text-primary">O4U Team</div>
                        <div class="text-caption text-medium-emphasis">Odoo & ERP Solutions</div>
                    </div>
                </div>

                <v-spacer></v-spacer>

                <div class="d-none d-md-flex align-center ga-6 mr-6">
                    <a v-for="item in navItems" :key="item.href" :href="item.href" class="landing-nav-link">
                        {{ item.label }}
                    </a>
                </div>

                <v-btn
                    v-if="user"
                    color="primary"
                    variant="flat"
                    class="text-none"
                    @click="router.visit('/dashboard')"
                >
                    Dashboard
                </v-btn>
                <v-btn
                    v-else
                    color="primary"
                    variant="outlined"
                    class="text-none"
                    @click="router.visit('/login')"
                >
                    Đăng nhập
                </v-btn>
            </v-container>
        </v-app-bar>

        <v-main class="landing-main">
            <slot />
        </v-main>

        <v-footer class="landing-footer">
            <v-container>
                <v-row align="center">
                    <v-col cols="12" md="6">
                        <div class="d-flex align-center ga-3 mb-3 mb-md-0">
                            <v-avatar color="primary" size="40" rounded="lg">
                                <span class="landing-logo-mark landing-logo-mark--sm font-weight-bold text-white">O4U</span>
                            </v-avatar>
                            <div>
                                <div class="font-weight-bold">O4U Team</div>
                                <div class="text-caption text-medium-emphasis">
                                    o4u.nvnhan0810.com
                                </div>
                            </div>
                        </div>
                    </v-col>
                    <v-col cols="12" md="6" class="text-md-end">
                        <div class="text-body-2 text-medium-emphasis">
                            Tư vấn & triển khai Odoo ERP theo nhu cầu doanh nghiệp
                        </div>
                        <div class="text-caption text-medium-emphasis mt-1">
                            © {{ currentYear }} O4U Team. All rights reserved.
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </v-footer>
    </v-app>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { router, usePage } from '@inertiajs/vue3';

interface PageProps extends Record<string, unknown> {
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
const currentYear = new Date().getFullYear();
const scrolled = ref(false);

const navItems = [
    { label: 'Dịch vụ', href: '#services' },
    { label: 'Quy trình', href: '#process' },
    { label: 'Về chúng tôi', href: '#about' },
    { label: 'Liên hệ', href: '#contact' },
];

const onScroll = () => {
    scrolled.value = window.scrollY > 24;
};

onMounted(() => {
    window.addEventListener('scroll', onScroll, { passive: true });
});

onUnmounted(() => {
    window.removeEventListener('scroll', onScroll);
});
</script>

<style scoped>
.landing-app-bar {
    background: rgba(255, 255, 255, 0.92) !important;
    backdrop-filter: blur(12px);
    border-bottom: 1px solid transparent;
    transition: box-shadow 0.2s ease, border-color 0.2s ease;
}

.landing-app-bar--scrolled {
    border-bottom-color: rgba(46, 125, 50, 0.12);
}

.landing-brand {
    cursor: pointer;
}

.landing-logo-mark {
    font-size: 0.8125rem;
    letter-spacing: -0.02em;
    line-height: 1;
}

.landing-logo-mark--sm {
    font-size: 0.75rem;
}

.landing-nav-link {
    color: rgba(0, 0, 0, 0.72);
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s ease;
}

.landing-nav-link:hover {
    color: rgb(var(--v-theme-primary));
}

.landing-main {
    background: #f7fbf7;
}

.landing-footer {
    background: #1b5e20 !important;
    color: white;
    padding: 32px 0;
}

.landing-footer .text-medium-emphasis {
    color: rgba(255, 255, 255, 0.78) !important;
}
</style>
