import { useAppRuntime } from '../../app/runtime';

export function useSession() {
    return useAppRuntime().session;
}
