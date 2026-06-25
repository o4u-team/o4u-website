<template>
    <div>
        <section class="hero-section">
            <v-container class="py-16 py-md-20">
                <v-row align="center">
                    <v-col cols="12" md="7">
                        <v-chip color="primary" variant="tonal" class="mb-4" size="small">
                            <v-icon start size="small">mdi-leaf</v-icon>
                            ERP Consultant
                        </v-chip>

                        <h1 class="hero-title mb-4">
                            Giải pháp ERP Odoo
                            <span class="text-primary">theo nhu cầu</span>
                            doanh nghiệp bạn
                        </h1>

                        <p class="hero-subtitle mb-8">
                            O4U Team đồng hành cùng doanh nghiệp trong tư vấn, triển khai và tùy biến
                            hệ thống Odoo — xây dựng nền tảng quản trị vận hành phù hợp với quy trình
                            thực tế của khách hàng.
                        </p>

                        <div class="d-flex flex-wrap ga-3">
                            <v-btn
                                color="primary"
                                size="large"
                                class="text-none px-6"
                                href="#contact"
                            >
                                Liên hệ tư vấn
                            </v-btn>
                            <v-btn
                                color="primary"
                                variant="outlined"
                                size="large"
                                class="text-none px-6"
                                href="#services"
                            >
                                Xem dịch vụ
                            </v-btn>
                        </div>
                    </v-col>

                    <v-col cols="12" md="5" class="d-none d-md-flex justify-center">
                        <div class="hero-visual">
                            <v-icon size="120" color="primary">mdi-office-building-cog</v-icon>
                            <div class="hero-visual__ring hero-visual__ring--outer"></div>
                            <div class="hero-visual__ring hero-visual__ring--inner"></div>
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="services" class="section-block">
            <v-container class="py-12 py-md-16">
                <div class="text-center mb-10">
                    <div class="section-label mb-2">Dịch vụ</div>
                    <h2 class="section-title">Chúng tôi làm gì?</h2>
                    <p class="section-desc mx-auto">
                        Từ phân tích nhu cầu đến vận hành ổn định, O4U Team hỗ trợ toàn diện
                        cho hành trình chuyển đổi số với Odoo ERP.
                    </p>
                </div>

                <v-row>
                    <v-col v-for="service in services" :key="service.title" cols="12" md="4">
                        <v-card class="service-card h-100" elevation="0" rounded="xl">
                            <v-card-text class="pa-6">
                                <v-avatar color="primary" variant="tonal" size="56" rounded="lg" class="mb-4">
                                    <v-icon :icon="service.icon" size="28"></v-icon>
                                </v-avatar>
                                <h3 class="text-h6 font-weight-bold mb-3">{{ service.title }}</h3>
                                <p class="text-body-2 text-medium-emphasis mb-0">
                                    {{ service.description }}
                                </p>
                            </v-card-text>
                        </v-card>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="process" class="section-block section-block--alt">
            <v-container class="py-12 py-md-16">
                <div class="text-center mb-10">
                    <div class="section-label mb-2">Quy trình</div>
                    <h2 class="section-title">Cách chúng tôi triển khai</h2>
                    <p class="section-desc mx-auto">
                        Quy trình rõ ràng, linh hoạt theo quy mô và ngành nghề của từng khách hàng.
                    </p>
                </div>

                <v-row>
                    <v-col v-for="(step, index) in processSteps" :key="step.title" cols="12" sm="6" lg="3">
                        <div class="process-step h-100">
                            <div class="process-step__number">{{ index + 1 }}</div>
                            <h3 class="text-subtitle-1 font-weight-bold mb-2">{{ step.title }}</h3>
                            <p class="text-body-2 text-medium-emphasis mb-0">{{ step.description }}</p>
                        </div>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="about" class="section-block">
            <v-container class="py-12 py-md-16">
                <v-row align="center">
                    <v-col cols="12" md="5">
                        <div class="section-label mb-2">Về O4U Team</div>
                        <h2 class="section-title mb-4">Đối tác tin cậy cho hệ thống ERP</h2>
                        <p class="text-body-1 text-medium-emphasis mb-4">
                            O4U Team tập trung vào việc giúp doanh nghiệp vận hành hiệu quả hơn thông qua
                            nền tảng Odoo — một hệ ERP mã nguồn mở mạnh mẽ, linh hoạt và có thể mở rộng.
                        </p>
                        <p class="text-body-1 text-medium-emphasis mb-0">
                            Chúng tôi không chỉ cài đặt phần mềm, mà thiết kế giải pháp bám sát quy trình
                            nghiệp vụ, giúp khách hàng số hóa bán hàng, kho, sản xuất, nhân sự, kế toán
                            và nhiều module khác theo đúng nhu cầu thực tế.
                        </p>
                    </v-col>

                    <v-col cols="12" md="7">
                        <v-row>
                            <v-col v-for="highlight in highlights" :key="highlight.title" cols="12" sm="6">
                                <div class="highlight-card">
                                    <v-icon :icon="highlight.icon" color="primary" size="28" class="mb-3"></v-icon>
                                    <div class="font-weight-bold mb-1">{{ highlight.title }}</div>
                                    <div class="text-body-2 text-medium-emphasis">{{ highlight.description }}</div>
                                </div>
                            </v-col>
                        </v-row>
                    </v-col>
                </v-row>
            </v-container>
        </section>

        <section id="contact" class="cta-section">
            <v-container class="py-12 py-md-16">
                <v-row justify="center">
                    <v-col cols="12" md="8" lg="7">
                        <v-card class="cta-card" rounded="xl" elevation="8">
                            <v-card-text class="pa-8 pa-md-10 text-center">
                                <v-icon color="white" size="48" class="mb-4">mdi-handshake-outline</v-icon>
                                <h2 class="text-h4 font-weight-bold text-white mb-4">
                                    Sẵn sàng bắt đầu dự án Odoo?
                                </h2>
                                <p class="text-body-1 text-white mb-6 cta-desc">
                                    Hãy chia sẻ nhu cầu ERP của bạn. O4U Team sẽ tư vấn lộ trình phù hợp
                                    để triển khai hệ thống theo quy mô và ngân sách doanh nghiệp.
                                </p>
                                <div class="d-flex flex-wrap justify-center ga-3">
                                    <v-btn
                                        color="white"
                                        size="large"
                                        class="text-none px-6 text-primary"
                                        href="mailto:nguyenvannhan0810@gmail.com"
                                    >
                                        <v-icon start>mdi-email-outline</v-icon>
                                        nguyenvannhan0810@gmail.com
                                    </v-btn>
                                </div>
                            </v-card-text>
                        </v-card>
                    </v-col>
                </v-row>
            </v-container>
        </section>
    </div>
</template>

<script setup lang="ts">
import LandingLayout from '@/Layouts/LandingLayout.vue';

defineOptions({
    layout: LandingLayout,
});

const services = [
    {
        icon: 'mdi-account-tie',
        title: 'Tư vấn Odoo ERP',
        description:
            'Phân tích quy trình nghiệp vụ, đề xuất kiến trúc hệ thống và lộ trình triển khai phù hợp với mục tiêu doanh nghiệp.',
    },
    {
        icon: 'mdi-cog-sync',
        title: 'Triển khai & Tùy biến',
        description:
            'Cài đặt Odoo, cấu hình module, phát triển tính năng theo yêu cầu riêng và tích hợp với hệ thống hiện có.',
    },
    {
        icon: 'mdi-database-cog',
        title: 'Xây dựng ERP theo nhu cầu',
        description:
            'Thiết kế giải pháp ERP end-to-end cho khách hàng — từ bán hàng, kho, sản xuất đến nhân sự và kế toán.',
    },
];

const processSteps = [
    {
        title: 'Khảo sát & Phân tích',
        description: 'Lắng nghe nhu cầu, mapping quy trình và xác định phạm vi triển khai.',
    },
    {
        title: 'Thiết kế giải pháp',
        description: 'Đề xuất module, luồng dữ liệu và kế hoạch triển khai chi tiết.',
    },
    {
        title: 'Triển khai & Tùy biến',
        description: 'Cấu hình Odoo, phát triển tính năng và kiểm thử theo nghiệp vụ thực tế.',
    },
    {
        title: 'Đào tạo & Hỗ trợ',
        description: 'Bàn giao hệ thống, đào tạo người dùng và đồng hành vận hành ổn định.',
    },
];

const highlights = [
    {
        icon: 'mdi-target',
        title: 'Bám sát nhu cầu',
        description: 'Giải pháp được thiết kế theo quy trình thực tế, không áp đặt template cứng nhắc.',
    },
    {
        icon: 'mdi-puzzle-outline',
        title: 'Odoo linh hoạt',
        description: 'Tận dụng sức mạnh mã nguồn mở để mở rộng và tùy biến theo thời gian.',
    },
    {
        icon: 'mdi-shield-check-outline',
        title: 'Triển khai tin cậy',
        description: 'Quy trình rõ ràng, kiểm soát chất lượng và bàn giao minh bạch.',
    },
    {
        icon: 'mdi-account-group-outline',
        title: 'Đồng hành lâu dài',
        description: 'Hỗ trợ sau triển khai để doanh nghiệp vận hành ERP hiệu quả.',
    },
];
</script>

<style scoped>
.hero-section {
    background:
        radial-gradient(circle at top right, rgba(76, 175, 80, 0.14), transparent 42%),
        linear-gradient(180deg, #ffffff 0%, #f7fbf7 100%);
}

.hero-title {
    font-size: clamp(2rem, 4vw, 3.25rem);
    line-height: 1.15;
    font-weight: 800;
    letter-spacing: -0.02em;
}

.hero-subtitle {
    font-size: 1.125rem;
    line-height: 1.7;
    color: rgba(0, 0, 0, 0.68);
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

.hero-visual__ring {
    position: absolute;
    border-radius: 50%;
    border: 2px solid rgba(46, 125, 50, 0.18);
}

.hero-visual__ring--outer {
    width: 100%;
    height: 100%;
    animation: pulse 4s ease-in-out infinite;
}

.hero-visual__ring--inner {
    width: 72%;
    height: 72%;
    border-color: rgba(76, 175, 80, 0.28);
}

@keyframes pulse {
    0%, 100% { transform: scale(1); opacity: 1; }
    50% { transform: scale(1.04); opacity: 0.72; }
}

.section-block {
    background: #ffffff;
}

.section-block--alt {
    background: #f3f9f3;
}

.section-label {
    color: rgb(var(--v-theme-primary));
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.8125rem;
}

.section-title {
    font-size: clamp(1.75rem, 3vw, 2.25rem);
    font-weight: 800;
    letter-spacing: -0.02em;
}

.section-desc {
    max-width: 640px;
    color: rgba(0, 0, 0, 0.64);
}

.service-card {
    border: 1px solid rgba(46, 125, 50, 0.12);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.service-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 32px rgba(46, 125, 50, 0.12) !important;
}

.process-step {
    background: white;
    border: 1px solid rgba(46, 125, 50, 0.1);
    border-radius: 20px;
    padding: 24px;
    height: 100%;
}

.process-step__number {
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: rgb(var(--v-theme-primary));
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    margin-bottom: 16px;
}

.highlight-card {
    background: #f7fbf7;
    border: 1px solid rgba(46, 125, 50, 0.1);
    border-radius: 16px;
    padding: 24px;
    height: 100%;
}

.cta-section {
    background: linear-gradient(180deg, #f7fbf7 0%, #e8f5e9 100%);
}

.cta-card {
    background: linear-gradient(135deg, #2e7d32 0%, #1b5e20 100%) !important;
}

.cta-desc {
    opacity: 0.92;
    max-width: 560px;
    margin-left: auto;
    margin-right: auto;
}
</style>
