export type ErpSaasRegisterModuleOption = {
    code: string;
    name: string;
};

export type ErpSaasRegisterPayload = {
    company_name: string;
    contact_name: string;
    email: string;
    phone: string;
    modules: string[];
    notes?: string;
    'cf-turnstile-response'?: string;
};

export type ErpSaasRegisterSuccessData = {
    tenant_url: string | null;
    email: string;
    email_sent: boolean;
    domain: string | null;
};

export type ErpSaasRegisterResponse = {
    success: boolean;
    message: string;
    code?: string | null;
    data?: ErpSaasRegisterSuccessData;
    errors?: Record<string, string[]>;
};

export const ERP_SAAS_MAX_MODULES = 2;

export function toggleRegisterModule(
    selected: string[],
    code: string,
    max = ERP_SAAS_MAX_MODULES,
): string[] {
    if (selected.includes(code)) {
        return selected.filter((c) => c !== code);
    }
    if (selected.length >= max) {
        return selected;
    }
    return [...selected, code];
}

export function mapRegisterFieldErrors(
    errors: Record<string, string[]> | undefined,
): Record<string, string> {
    if (!errors) {
        return {};
    }
    const mapped: Record<string, string> = {};
    for (const [key, messages] of Object.entries(errors)) {
        const first = messages[0];
        if (first) {
            mapped[key] = first;
        }
    }
    return mapped;
}
