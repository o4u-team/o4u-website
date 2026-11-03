export type ProductLink = {
    key: string;
    name: string;
    tagline: string;
    url: string;
    navLabel: string;
};

function readString(record: object, key: string): string | null {
    if (!(key in record)) {
        return null;
    }
    const value: unknown = Reflect.get(record, key);
    return typeof value === 'string' ? value : null;
}

function isProductLink(value: unknown): value is ProductLink {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    return (
        readString(value, 'key') !== null &&
        readString(value, 'name') !== null &&
        readString(value, 'tagline') !== null &&
        readString(value, 'url') !== null &&
        readString(value, 'navLabel') !== null
    );
}

export function parseProductLinks(value: unknown): ProductLink[] {
    if (!Array.isArray(value)) {
        return [];
    }
    return value.filter(isProductLink);
}
