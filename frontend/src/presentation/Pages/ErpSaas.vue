<template>
    <div class="erp-saas">
        <section class="hero-section">
            <v-container class="py-16 py-md-20">
                <v-row align="center">
                    <v-col cols="12" md="7">
                        <span class="eyebrow">
                            <v-icon
                                start
                                size="small"
                                icon="mdi-cloud-outline"
                            ></v-icon>
                            ERP SaaS
                        </span>

                        <h1 class="hero-title mb-4">
                            ERP trên web
                            <span class="accent">cho từng doanh nghiệp</span>
                        </h1>

                        <p class="hero-subtitle mb-8">
                            Cổng quản trị vận hành bán hàng, mua hàng, tồn kho
                            và nhân sự — mỗi doanh nghiệp làm việc trên domain
                            riêng, phân quyền theo module, không cần vào backend
                            ERP để thao tác hàng ngày.
                        </p>

                        <div class="d-flex flex-wrap ga-3">
                            <a class="btn btn--solid" href="#register"
                                >Đăng ký dùng thử</a
                            >
                            <a class="btn btn--outline" href="#modules"
                                >Xem module</a
                            >
                        </div>
                    </v-col>

                    <v-col
                        cols="12"
                        md="5"
                        class="d-none d-md-flex justify-center"
                    >
                        <div class="hero-visual">
                            <v-icon
                                size="120"
                                class="hero-visual__icon"
                                icon="mdi-application-cog-outline"
                            ></v-icon>
                            <div
                                class="hero-visual__ring hero-visual__ring--outer"
                            ></div>
                            <div
                                class="hero-visual__ring hero-visual__ring--inner"
                            ></div>
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="features" class="section-block">
            <v-container class="py-12 py-md-16">
                <div class="text-center mb-10">
                    <div class="section-label mb-2">Tính năng</div>
                    <h2 class="section-title">Vì sao chọn ERP SaaS?</h2>
                    <p class="section-desc mx-auto">
                        Tập trung vào công việc vận hành hàng ngày — nhanh, rõ
                        ràng, đúng quyền hạn.
                    </p>
                </div>

                <v-row>
                    <v-col
                        v-for="feature in features"
                        :key="feature.title"
                        cols="12"
                        md="4"
                    >
                        <article class="card h-100">
                            <div class="card__icon" aria-hidden="true">
                                <v-icon :icon="feature.icon" size="28"></v-icon>
                            </div>
                            <h3 class="text-h6 font-weight-bold mb-3">
                                {{ feature.title }}
                            </h3>
                            <p class="text-body-2 text-medium-emphasis mb-0">
                                {{ feature.description }}
                            </p>
                        </article>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="modules" class="section-block section-block--alt">
            <v-container class="py-12 py-md-16">
                <div class="text-center mb-10">
                    <div class="section-label mb-2">Module</div>
                    <h2 class="section-title">Làm việc theo nghiệp vụ</h2>
                    <p class="section-desc mx-auto">
                        Các module chính trên cổng web, bám sát quy trình bán —
                        mua — kho — nhân sự.
                    </p>
                </div>

                <v-row>
                    <v-col
                        v-for="mod in modules"
                        :key="mod.title"
                        cols="12"
                        sm="6"
                        lg="3"
                    >
                        <article class="card h-100">
                            <div
                                class="card__icon card__icon--sm"
                                aria-hidden="true"
                            >
                                <v-icon :icon="mod.icon" size="24"></v-icon>
                            </div>
                            <h3 class="text-subtitle-1 font-weight-bold mb-2">
                                {{ mod.title }}
                            </h3>
                            <p class="text-body-2 text-medium-emphasis mb-3">
                                {{ mod.description }}
                            </p>
                            <ul class="module-list">
                                <li v-for="item in mod.items" :key="item">
                                    {{ item }}
                                </li>
                            </ul>
                        </article>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="workflow" class="section-block">
            <v-container class="py-12 py-md-16">
                <div class="text-center mb-10">
                    <div class="section-label mb-2">Luồng vận hành</div>
                    <h2 class="section-title">Từ đơn hàng đến tồn kho</h2>
                    <p class="section-desc mx-auto">
                        Một chuỗi nghiệp vụ liền mạch trên web — đúng người,
                        đúng quyền, đúng bước.
                    </p>
                </div>

                <v-row>
                    <v-col
                        v-for="(step, index) in workflowSteps"
                        :key="step.title"
                        cols="12"
                        sm="6"
                        lg="3"
                    >
                        <div class="step h-100">
                            <div class="step__number">{{ index + 1 }}</div>
                            <h3 class="text-subtitle-1 font-weight-bold mb-2">
                                {{ step.title }}
                            </h3>
                            <p class="text-body-2 text-medium-emphasis mb-0">
                                {{ step.description }}
                            </p>
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="register" class="section-block section-block--alt">
            <v-container class="py-12 py-md-16">
                <ErpSaasRegisterForm
                    v-if="registerConfig"
                    :submit-url="registerConfig.submitUrl"
                    :turnstile-site-key="registerConfig.turnstileSiteKey"
                    :turnstile-bypass="registerConfig.turnstileBypass"
                    :modules="registerConfig.modules"
                    :max-modules="registerConfig.maxModules"
                />
            </v-container>
        </section>

        <section id="contact" class="cta-section">
            <v-container class="py-12 py-md-16">
                <v-row justify="center">
                    <v-col cols="12" md="8" lg="7">
                        <div class="cta-card text-center">
                            <v-icon
                                size="48"
                                class="mb-4 cta-card__icon"
                                icon="mdi-rocket-launch-outline"
                            ></v-icon>
                            <h2
                                class="text-h4 font-weight-bold text-white mb-4"
                            >
                                Muốn dùng thử ERP SaaS?
                            </h2>
                            <p class="text-body-1 text-white mb-6 cta-desc">
                                Chia sẻ nhu cầu vận hành của bạn. O4U Team sẽ tư
                                vấn cấu hình module và lộ trình triển khai phù
                                hợp quy mô doanh nghiệp.
                            </p>
                            <div class="d-flex flex-wrap justify-center ga-3">
                                <a
                                    v-if="contact"
                                    class="btn btn--icon btn--on-dark"
                                    :href="contact.emailHref"
                                    :aria-label="`Email ${contact.email}`"
                                >
                                    <v-icon
                                        size="22"
                                        icon="mdi-email-outline"
                                    ></v-icon>
                                </a>
                                <a
                                    v-if="contact"
                                    class="btn btn--icon btn--on-dark"
                                    :href="contact.phoneHref"
                                    :aria-label="`Gọi ${contact.phone}`"
                                >
                                    <v-icon size="22" icon="mdi-phone"></v-icon>
                                </a>
                                <button
                                    v-if="contact"
                                    type="button"
                                    class="btn btn--icon btn--on-dark"
                                    :aria-label="`Zalo ${contact.phone}`"
                                    @click="zaloOpen = true"
                                >
                                    <ZaloIcon />
                                </button>
                                <button
                                    type="button"
                                    class="btn btn--ghost-on-dark"
                                    @click="router.push('/')"
                                >
                                    Về trang chủ O4U
                                </button>
                            </div>
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <ZaloQrDialog
            v-if="contact"
            v-model="zaloOpen"
            :qr-url="contact.zaloQrUrl"
            :display-name="contact.zaloDisplayName"
        />
    </div>
</template>

<script setup lang="ts">
import ErpSaasRegisterForm from '@/presentation/Components/marketing/ErpSaasRegisterForm.vue';
import ZaloIcon from '@/presentation/Components/ZaloIcon.vue';
import ZaloQrDialog from '@/presentation/Components/ZaloQrDialog.vue';
import { parseContactInfo } from '@/domain/marketing/contact-info';
import { usePublicContent } from '@/presentation/composables/public-content';
import { computed, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const page = usePublicContent();
const contact = computed(() => parseContactInfo(page.props.contact));
const registerConfig = computed(() => {
    const raw = page.props.erpSaasRegister;
    if (!raw || typeof raw.submitUrl !== 'string') {
        return null;
    }
    return {
        submitUrl: raw.submitUrl,
        turnstileSiteKey: raw.turnstileSiteKey ?? null,
        turnstileBypass: Boolean(raw.turnstileBypass),
        modules: Array.isArray(raw.modules) ? raw.modules : [],
        maxModules: typeof raw.maxModules === 'number' ? raw.maxModules : 2,
    };
});
const zaloOpen = ref(false);

const features = [
    {
        icon: 'mdi-domain',
        title: 'Domain riêng từng DN',
        description:
            'Mỗi doanh nghiệp truy cập trên subdomain riêng. Dữ liệu và branding tách biệt theo tenant.',
    },
    {
        icon: 'mdi-shield-account-outline',
        title: 'Phân quyền theo module',
        description:
            'Chỉ mở đúng chức năng cần thiết: đọc, tạo, xác nhận đơn, xuất kho, quản lý nhân viên…',
    },
    {
        icon: 'mdi-monitor-dashboard',
        title: 'Làm việc trên web',
        description:
            'Nhân viên thao tác bán hàng, mua hàng, kho và nhân sự trên cổng web — không cần vào backend.',
    },
];

const modules = [
    {
        icon: 'mdi-cart-outline',
        title: 'Bán hàng',
        description: 'Quản lý đơn bán và khách hàng trên web.',
        items: ['Đơn hàng', 'Khách hàng', 'Hóa đơn bán', 'Sản phẩm bán'],
    },
    {
        icon: 'mdi-bag-personal-outline',
        title: 'Mua hàng',
        description: 'Theo dõi đơn mua và nhà cung cấp.',
        items: ['Đơn mua', 'Nhà cung cấp', 'Hóa đơn mua', 'Bảng giá NCC'],
    },
    {
        icon: 'mdi-warehouse',
        title: 'Tồn kho',
        description: 'Phiếu kho, tồn thực tế và sản phẩm.',
        items: [
            'Nhập / Xuất / Nội bộ',
            'Tồn kho',
            'Sản phẩm & biến thể',
            'Lot / Serial',
        ],
    },
    {
        icon: 'mdi-account-group-outline',
        title: 'Nhân sự',
        description: 'Hồ sơ nhân viên và quyền truy cập.',
        items: [
            'Danh sách nhân viên',
            'Tạo / cập nhật',
            'Phân quyền module',
            'Reset mật khẩu',
        ],
    },
];

const workflowSteps = [
    {
        title: 'Tạo đơn bán / mua',
        description:
            'Nhân viên lập đơn trên web, chọn khách hàng hoặc nhà cung cấp.',
    },
    {
        title: 'Xác nhận nghiệp vụ',
        description:
            'Người có quyền xác nhận đơn, tạo hóa đơn hoặc phiếu kho liên quan.',
    },
    {
        title: 'Xuất / nhập kho',
        description:
            'Theo dõi phiếu nhập, xuất, nội bộ và điều chỉnh tồn khi cần.',
    },
    {
        title: 'Kiểm soát quyền',
        description:
            'Admin gán module và tính năng cho từng nhân viên theo vai trò.',
    },
];
</script>

<style scoped>
.erp-saas {
    --erp: #0f766e;
    --erp-strong: #0d9488;
    --erp-deep: #115e59;
    --erp-soft: #f0fdfa;
    --erp-line: rgba(13, 148, 136, 0.22);
    --erp-ink: #134e4a;
}

.eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    margin-bottom: 1rem;
    padding: 0.35rem 0.75rem;
    border-radius: 999px;
    background: rgba(13, 148, 136, 0.12);
    color: var(--erp);
    font-weight: 700;
    font-size: 0.8125rem;
}

.hero-section {
    background:
        radial-gradient(
            circle at top right,
            rgba(13, 148, 136, 0.16),
            transparent 42%
        ),
        linear-gradient(180deg, #ffffff 0%, #f0fdfa 100%);
}

.hero-title {
    font-size: clamp(2rem, 4vw, 3.25rem);
    line-height: 1.15;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--erp-ink);
}

.accent {
    color: var(--erp);
}

.hero-subtitle {
    font-size: 1.125rem;
    line-height: 1.7;
    color: rgba(19, 78, 74, 0.72);
    max-width: 640px;
}

.hero-visual {
    position: relative;
    width: 280px;
    height: 280px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.hero-visual__icon {
    color: var(--erp) !important;
}

.hero-visual__ring {
    position: absolute;
    border-radius: 50%;
    border: 2px solid rgba(15, 118, 110, 0.2);
}

.hero-visual__ring--outer {
    width: 100%;
    height: 100%;
    animation: pulse 4s ease-in-out infinite;
}

.hero-visual__ring--inner {
    width: 72%;
    height: 72%;
    border-color: rgba(13, 148, 136, 0.32);
}

@keyframes pulse {
    0%,
    100% {
        transform: scale(1);
        opacity: 1;
    }
    50% {
        transform: scale(1.04);
        opacity: 0.72;
    }
}

.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    min-height: 2.75rem;
    padding: 0.55rem 1.4rem;
    border-radius: 0.65rem;
    font-weight: 700;
    text-decoration: none;
    cursor: pointer;
    border: 1px solid transparent;
    font: inherit;
}

.btn--solid {
    background: var(--erp);
    color: #fff;
}

.btn--solid:hover {
    background: var(--erp-deep);
}

.btn--outline {
    background: transparent;
    color: var(--erp);
    border-color: var(--erp);
}

.btn--outline:hover {
    background: var(--erp-soft);
}

.btn--on-dark {
    background: #fff;
    color: var(--erp-deep);
}

.btn--icon {
    width: 3rem;
    min-width: 3rem;
    padding: 0;
}

.btn--ghost-on-dark {
    background: transparent;
    color: #fff;
    border-color: rgba(255, 255, 255, 0.55);
}

.section-block {
    background: #ffffff;
}

.section-block--alt {
    background: #ecfdf8;
}

.section-label {
    color: var(--erp);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.8125rem;
}

.section-title {
    font-size: clamp(1.75rem, 3vw, 2.25rem);
    font-weight: 800;
    letter-spacing: -0.02em;
    color: var(--erp-ink);
}

.section-desc {
    max-width: 640px;
    color: rgba(19, 78, 74, 0.68);
}

.card,
.step {
    background: #fff;
    border: 1px solid var(--erp-line);
    border-radius: 1.15rem;
    padding: 1.4rem 1.3rem;
    height: 100%;
    transition:
        transform 0.2s ease,
        box-shadow 0.2s ease;
}

.card:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(15, 118, 110, 0.12);
}

.card__icon {
    width: 3.5rem;
    height: 3.5rem;
    border-radius: 0.85rem;
    display: grid;
    place-items: center;
    background: rgba(13, 148, 136, 0.12);
    color: var(--erp);
    margin-bottom: 1rem;
}

.card__icon--sm {
    width: 3rem;
    height: 3rem;
}

.module-list {
    margin: 0;
    padding-left: 1.1rem;
    color: rgba(19, 78, 74, 0.68);
    font-size: 0.875rem;
    line-height: 1.55;
}

.step__number {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 0.75rem;
    display: grid;
    place-items: center;
    background: var(--erp);
    color: #fff;
    font-weight: 800;
    margin-bottom: 1rem;
}

.cta-section {
    background: linear-gradient(180deg, #f0fdfa 0%, #ccfbf1 100%);
}

.cta-card {
    background: linear-gradient(135deg, var(--erp-deep) 0%, var(--erp) 100%);
    border-radius: 1.4rem;
    padding: 2.5rem 1.5rem;
}

.cta-card__icon {
    color: #fff !important;
}

.cta-desc {
    opacity: 0.92;
    max-width: 560px;
    margin-left: auto;
    margin-right: auto;
}

@media (prefers-reduced-motion: reduce) {
    .hero-visual__ring--outer,
    .card {
        animation: none;
        transition: none;
    }
}
</style>
