import { computed } from 'vue';
import { useRoute } from 'vue-router';
import type { Resource, SettingKey } from '../../domain/entities';
export const labels: Record<Resource, string> = {
    clients: 'Khách hàng',
    apps: 'Ứng dụng',
    'client-systems': 'Hệ thống khách hàng',
};
export const settingLabels: Record<SettingKey, string> = {
    webapp_url: 'Webapp URL',
    android_min_version: 'Android · phiên bản tối thiểu',
    android_current_version: 'Android · phiên bản hiện tại',
    ios_min_version: 'iOS · phiên bản tối thiểu',
    ios_current_version: 'iOS · phiên bản hiện tại',
    android_store_url: 'Google Play URL',
    apple_store_url: 'App Store URL',
};
export function useResource() {
    const route = useRoute();
    const resource = computed<Resource>(() => {
        const value = route.params.resource;
        if (
            value === 'clients' ||
            value === 'apps' ||
            value === 'client-systems'
        )
            return value;
        throw new Error('Unknown resource');
    });
    return { resource, title: computed(() => labels[resource.value]) };
}
export function errorMessage(error: unknown): string {
    return error instanceof Error
        ? error.message
        : 'Không thể kết nối máy chủ.';
}
