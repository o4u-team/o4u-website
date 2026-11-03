<template>
    <section :id="sectionId" class="erp-register">
        <div class="erp-register__inner">
            <header
                v-if="heading || subheading"
                class="erp-register__header text-center mb-8"
            >
                <div v-if="eyebrow" class="section-label mb-2">
                    {{ eyebrow }}
                </div>
                <h2 v-if="heading" class="section-title">{{ heading }}</h2>
                <p v-if="subheading" class="section-desc mx-auto">
                    {{ subheading }}
                </p>
            </header>

            <div v-if="done" class="erp-register__success text-center">
                <v-icon
                    size="48"
                    color="success"
                    icon="mdi-check-circle-outline"
                    class="mb-3"
                ></v-icon>
                <h3 class="text-h6 font-weight-bold mb-2">
                    Đăng ký thành công
                </h3>
                <p class="text-body-2 text-medium-emphasis mb-4">
                    {{ successMessage }}
                </p>
                <a
                    v-if="tenantUrl"
                    class="btn btn--solid"
                    :href="tenantUrl"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    Mở cổng ERP
                </a>
            </div>

            <v-form v-else ref="formRef" @submit.prevent="onSubmit">
                <v-alert
                    v-if="formError"
                    type="error"
                    variant="tonal"
                    class="mb-4"
                    density="comfortable"
                >
                    {{ formError }}
                </v-alert>

                <v-text-field
                    v-model="form.company_name"
                    label="Tên công ty"
                    :error-messages="fieldErrors.company_name"
                    :disabled="submitting"
                    required
                    class="mb-1"
                ></v-text-field>
                <v-text-field
                    v-model="form.contact_name"
                    label="Họ và tên"
                    :error-messages="fieldErrors.contact_name"
                    :disabled="submitting"
                    required
                    class="mb-1"
                ></v-text-field>
                <v-text-field
                    v-model="form.email"
                    label="Email (dùng để đăng nhập)"
                    type="email"
                    :error-messages="fieldErrors.email"
                    :disabled="submitting"
                    required
                    class="mb-1"
                ></v-text-field>
                <v-text-field
                    v-model="form.phone"
                    label="Số điện thoại"
                    :error-messages="fieldErrors.phone"
                    :disabled="submitting"
                    required
                    class="mb-1"
                ></v-text-field>

                <div class="erp-register__modules mb-4">
                    <div
                        v-for="mod in modules"
                        :key="mod.code"
                        class="erp-register__module"
                        :class="{
                            'erp-register__module--on':
                                selectedModules.includes(mod.code),
                            'erp-register__module--disabled':
                                !selectedModules.includes(mod.code) &&
                                selectedModules.length >= maxModules,
                        }"
                        @click="onToggleModule(mod.code)"
                    >
                        <v-checkbox
                            :model-value="selectedModules.includes(mod.code)"
                            :label="mod.name"
                            :disabled="
                                submitting ||
                                (!selectedModules.includes(mod.code) &&
                                    selectedModules.length >= maxModules)
                            "
                            density="compact"
                            hide-details
                            readonly
                            class="flex-grow-0 ma-0"
                        ></v-checkbox>
                    </div>
                    <div
                        v-if="fieldErrors.modules"
                        class="text-error text-caption mt-1"
                    >
                        {{ fieldErrors.modules }}
                    </div>
                </div>

                <div
                    v-if="!turnstileBypass && turnstileSiteKey"
                    class="erp-register__captcha mb-4"
                >
                    <div ref="turnstileEl"></div>
                    <div
                        v-if="fieldErrors['cf-turnstile-response']"
                        class="text-error text-caption mt-1"
                    >
                        {{ fieldErrors['cf-turnstile-response'] }}
                    </div>
                </div>

                <div class="d-flex justify-center mt-2">
                    <button
                        type="submit"
                        class="btn btn--solid"
                        :disabled="submitting"
                    >
                        {{ submitting ? 'Đang tạo…' : submitLabel }}
                    </button>
                </div>
            </v-form>
        </div>
    </section>
</template>

<script setup lang="ts">
import {
    mapRegisterFieldErrors,
    toggleRegisterModule,
    type ErpSaasRegisterModuleOption,
    type ErpSaasRegisterPayload,
} from '@/domain/marketing/erp-saas-register';
import { onBeforeUnmount, onMounted, reactive, ref } from 'vue';
import { useO4u } from '@/app/runtime';

const props = withDefaults(
    defineProps<{
        submitUrl: string;
        turnstileSiteKey?: string | null;
        turnstileBypass?: boolean;
        modules?: ErpSaasRegisterModuleOption[];
        maxModules?: number;
        sectionId?: string;
        eyebrow?: string;
        heading?: string;
        subheading?: string;
        submitLabel?: string;
    }>(),
    {
        turnstileSiteKey: null,
        turnstileBypass: false,
        modules: () => [],
        maxModules: 2,
        sectionId: 'register',
        eyebrow: 'Dùng thử',
        heading: 'Đăng ký ERP SaaS',
        subheading:
            'Tạo tenant dùng thử — thông tin đăng nhập sẽ gửi về email của bạn.',
        submitLabel: 'Tạo tài khoản dùng thử',
    },
);
const o4u = useO4u();

const emit = defineEmits<{
    success: [payload: { tenant_url: string | null; email: string }];
}>();

const form = reactive({
    company_name: '',
    contact_name: '',
    email: '',
    phone: '',
});

const selectedModules = ref<string[]>([]);
const fieldErrors = reactive<Record<string, string>>({});
const formError = ref('');
const submitting = ref(false);
const done = ref(false);
const successMessage = ref('');
const tenantUrl = ref<string | null>(null);
const turnstileToken = ref('');
const turnstileEl = ref<HTMLElement | null>(null);
let turnstileWidgetId: string | null = null;

function onToggleModule(code: string): void {
    if (submitting.value) {
        return;
    }
    selectedModules.value = toggleRegisterModule(
        selectedModules.value,
        code,
        props.maxModules,
    );
    delete fieldErrors.modules;
}

type TurnstileApi = {
    render: (
        el: HTMLElement,
        opts: {
            sitekey: string;
            callback: (token: string) => void;
            'expired-callback'?: () => void;
            'error-callback'?: () => void;
        },
    ) => string;
    reset: (widgetId?: string) => void;
    remove: (widgetId?: string) => void;
};

function getTurnstile(): TurnstileApi | undefined {
    return (window as unknown as { turnstile?: TurnstileApi }).turnstile;
}

function clearErrors(): void {
    formError.value = '';
    for (const key of Object.keys(fieldErrors)) {
        delete fieldErrors[key];
    }
}

function applyErrors(
    errors: Record<string, string[]> | undefined,
    message: string,
): void {
    clearErrors();
    Object.assign(fieldErrors, mapRegisterFieldErrors(errors));
    formError.value = message;
}

function loadTurnstileScript(): Promise<void> {
    if (getTurnstile()) {
        return Promise.resolve();
    }
    return new Promise((resolve, reject) => {
        const existing = document.querySelector<HTMLScriptElement>(
            'script[data-turnstile]',
        );
        if (existing) {
            existing.addEventListener('load', () => resolve());
            existing.addEventListener('error', () =>
                reject(new Error('Turnstile load failed')),
            );
            return;
        }
        const script = document.createElement('script');
        script.src =
            'https://challenges.cloudflare.com/turnstile/v0/api.js?render=explicit';
        script.async = true;
        script.dataset.turnstile = '1';
        script.onload = () => resolve();
        script.onerror = () => reject(new Error('Turnstile load failed'));
        document.head.appendChild(script);
    });
}

async function mountTurnstile(): Promise<void> {
    if (
        props.turnstileBypass ||
        !props.turnstileSiteKey ||
        !turnstileEl.value
    ) {
        return;
    }
    await loadTurnstileScript();
    const api = getTurnstile();
    if (!api || !turnstileEl.value) {
        return;
    }
    turnstileWidgetId = api.render(turnstileEl.value, {
        sitekey: props.turnstileSiteKey,
        callback: (token: string) => {
            turnstileToken.value = token;
            delete fieldErrors['cf-turnstile-response'];
        },
        'expired-callback': () => {
            turnstileToken.value = '';
        },
        'error-callback': () => {
            turnstileToken.value = '';
        },
    });
}

async function onSubmit(): Promise<void> {
    clearErrors();
    if (selectedModules.value.length < 1) {
        fieldErrors.modules = 'Vui lòng chọn ít nhất 1 module.';
        return;
    }
    if (selectedModules.value.length > props.maxModules) {
        fieldErrors.modules = `Chỉ được chọn tối đa ${props.maxModules} module.`;
        return;
    }
    if (
        !props.turnstileBypass &&
        props.turnstileSiteKey &&
        !turnstileToken.value
    ) {
        fieldErrors['cf-turnstile-response'] = 'Vui lòng xác minh captcha.';
        return;
    }

    submitting.value = true;
    const payload: ErpSaasRegisterPayload = {
        company_name: form.company_name.trim(),
        contact_name: form.contact_name.trim(),
        email: form.email.trim(),
        phone: form.phone.trim(),
        modules: [...selectedModules.value],
    };
    if (!props.turnstileBypass) {
        payload['cf-turnstile-response'] = turnstileToken.value;
    }

    try {
        const body = await o4u.repository.register(payload);
        if (!body.success) {
            applyErrors(
                body.errors,
                body.message || 'Đăng ký thất bại. Vui lòng thử lại.',
            );
            if (turnstileWidgetId && getTurnstile()) {
                getTurnstile()?.reset(turnstileWidgetId);
                turnstileToken.value = '';
            }
            return;
        }

        done.value = true;
        successMessage.value =
            body.message ||
            'Đã tạo tenant. Vui lòng kiểm tra email để nhận thông tin đăng nhập.';
        tenantUrl.value = body.data?.tenant_url ?? null;
        emit('success', {
            tenant_url: tenantUrl.value,
            email: body.data?.email ?? form.email,
        });
    } catch {
        formError.value = 'Không kết nối được máy chủ. Vui lòng thử lại.';
    } finally {
        submitting.value = false;
    }
}

onMounted(() => {
    void mountTurnstile().catch(() => {
        formError.value = 'Không tải được captcha. Vui lòng tải lại trang.';
    });
});

onBeforeUnmount(() => {
    if (turnstileWidgetId && getTurnstile()) {
        getTurnstile()?.remove(turnstileWidgetId);
    }
});
</script>

<style scoped>
.erp-register {
    padding: 3rem 0;
}

.erp-register__inner {
    max-width: 32rem;
    margin: 0 auto;
    padding: 1.5rem;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.92);
    border: 1px solid rgba(15, 118, 110, 0.12);
    box-shadow: 0 12px 40px rgba(15, 118, 110, 0.08);
}

.erp-register__modules {
    text-align: left;
}

.erp-register__module {
    display: flex;
    align-items: center;
    padding: 0.35rem 0.65rem;
    margin-bottom: 0.4rem;
    border-radius: 10px;
    border: 1px solid rgba(15, 118, 110, 0.18);
    cursor: pointer;
    transition:
        border-color 0.15s ease,
        background 0.15s ease;
}

.erp-register__module--on {
    border-color: #0f766e;
    background: rgba(15, 118, 110, 0.06);
}

.erp-register__module--disabled {
    opacity: 0.55;
    cursor: not-allowed;
}

.erp-register__captcha {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.section-label {
    font-size: 0.75rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    font-weight: 700;
    color: #0f766e;
}

.section-title {
    font-size: 1.75rem;
    font-weight: 800;
    color: #134e4a;
}

.section-desc {
    max-width: 36rem;
    color: rgba(0, 0, 0, 0.55);
}

.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    border-radius: 999px;
    font-weight: 700;
    text-decoration: none;
    border: none;
    cursor: pointer;
}

.btn:disabled {
    opacity: 0.65;
    cursor: not-allowed;
}

.btn--solid {
    background: #0f766e;
    color: #fff;
}

.btn--solid:hover:not(:disabled) {
    background: #0d9488;
}

.text-error {
    color: #b91c1c;
}
</style>
