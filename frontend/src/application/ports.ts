import type {
    Assignments,
    Draft,
    Entity,
    ListFilters,
    Page,
    PublicConfig,
    Resource,
    SyncApps,
    Usage,
    User,
} from '../domain/entities';
import type {
    ErpSaasRegisterPayload,
    ErpSaasRegisterResponse,
} from '../domain/marketing/erp-saas-register';
export interface O4uRepository {
    me(): Promise<User>;
    logout(): Promise<void>;
    list(resource: Resource, filters: ListFilters): Promise<Page<Entity>>;
    get(resource: Resource, id: number): Promise<Entity>;
    save(resource: Resource, id: number | null, draft: Draft): Promise<Entity>;
    remove(resource: Resource, id: number): Promise<void>;
    assignments(id: number): Promise<Assignments>;
    syncApps(id: number, data: SyncApps): Promise<void>;
    usage(filters: Record<string, string | number>): Promise<Usage>;
    publicConfig(): Promise<PublicConfig>;
    register(data: ErpSaasRegisterPayload): Promise<ErpSaasRegisterResponse>;
}
