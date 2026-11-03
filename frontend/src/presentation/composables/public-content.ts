import { computed, reactive } from 'vue';
import { useRoute } from 'vue-router';
import { useAppRuntime } from '../../app/runtime';
import { useSession } from './session';

export function usePublicContent() {
    const route = useRoute();
    const { user } = useSession();
    const { publicConfig } = useAppRuntime();
    return reactive({
        url: computed(() => route.fullPath),
        props: {
            auth: { user },
            products: [
                {
                    key: 'erp_saas',
                    name: 'ERP SaaS',
                    tagline: 'Cổng quản trị ERP trên web cho từng doanh nghiệp',
                    navLabel: 'Sản phẩm',
                    url: '/erp-saas',
                },
            ],
            contact: {
                email: 'nguyenvannhan0810@gmail.com',
                emailHref: 'mailto:nguyenvannhan0810@gmail.com',
                phone: '0799833537',
                phoneHref: 'tel:+84799833537',
                zaloHref: 'https://zalo.me/0799833537',
                zaloQrUrl: '/images/contact/zalo-qr.jpg',
                zaloDisplayName: 'Nguyễn Văn Nhàn',
            },
            erpSaasRegister: {
                submitUrl: '/erp-saas/register',
                turnstileSiteKey: publicConfig.turnstileSiteKey,
                turnstileBypass: publicConfig.turnstileBypass,
                modules: [
                    { code: 'inventory', name: 'Tồn kho' },
                    { code: 'sale', name: 'Bán hàng' },
                    { code: 'purchase', name: 'Mua hàng' },
                    { code: 'employee', name: 'Nhân viên' },
                ],
                maxModules: 2,
            },
        },
    });
}
