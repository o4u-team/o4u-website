export type Resource = 'clients' | 'apps' | 'client-systems';
export const settingKeys = [
    'webapp_url',
    'android_min_version',
    'android_current_version',
    'ios_min_version',
    'ios_current_version',
    'android_store_url',
    'apple_store_url',
] as const;
export type SettingKey = (typeof settingKeys)[number];
export type AppSettings = Record<SettingKey, string | null>;
export interface Entity {
    id: number;
    name: string;
    uuid: string;
    status: string;
    created_at?: string | null;
    updated_at?: string | null;
    client_id?: number;
    endpoint?: string;
    webapp_endpoint?: string | null;
    db_name?: string;
    allow_get_info?: boolean;
    allow_public?: boolean;
    client?: Entity | null;
    webapp_url?: string | null;
    android_min_version?: string | null;
    android_current_version?: string | null;
    ios_min_version?: string | null;
    ios_current_version?: string | null;
    android_store_url?: string | null;
    apple_store_url?: string | null;
}
export interface User {
    id: number;
    name: string;
    email: string;
    avatar: string | null;
}
export interface Page<T> {
    data: T[];
    total: number;
    current_page: number;
    last_page: number;
}
export interface ListFilters {
    search: string;
    status: string;
    page: number;
    sort_field: string;
    sort_direction: 'asc' | 'desc';
    client_id?: number;
}
export interface AssignedApp extends Entity {
    assigned: boolean;
    overrides: Partial<AppSettings> | null;
}
export interface Assignments {
    clientSystem: Entity;
    apps: AssignedApp[];
}
export interface Device {
    id: number;
    username: string;
    app_version: string | null;
    last_connected_at: string | null;
    app: Entity;
    client_system: Entity;
    user_device: { device_id: string; device_info: unknown };
}
export interface ConnectionLog {
    id: number;
    app_version: string | null;
    ip_address: string | null;
    created_at: string | null;
    app_user_device: Device;
}
export interface Usage {
    appUserDevices: Page<Device>;
    connectionLogs: Page<ConnectionLog>;
    apps: Entity[];
    clientSystems: Entity[];
}
export type Draft = Record<string, string | number | boolean | null>;
export interface PublicConfig {
    turnstileSiteKey: string;
    turnstileBypass: boolean;
}
export interface SyncApps {
    app_ids: number[];
    app_settings: Record<string, Partial<AppSettings>>;
}
export class ApiFailure extends Error {
    constructor(
        message: string,
        public readonly status: number,
        public readonly errors: Record<string, string[]> = {},
    ) {
        super(message);
    }
}
