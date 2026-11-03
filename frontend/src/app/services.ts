import { O4u } from '../application/o4u';
import { HttpRepository } from '../infrastructure/http-repository';
export function createO4u(base = '', cookie?: string): O4u {
    return new O4u(new HttpRepository(base, cookie));
}
