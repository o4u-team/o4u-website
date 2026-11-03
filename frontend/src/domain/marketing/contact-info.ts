export type ContactInfo = {
    email: string;
    emailHref: string;
    phone: string;
    phoneHref: string;
    zaloHref: string;
    zaloQrUrl: string;
    zaloDisplayName: string;
};

function readString(record: object, key: string): string | null {
    if (!(key in record)) {
        return null;
    }
    const value: unknown = Reflect.get(record, key);
    return typeof value === 'string' ? value : null;
}

export function parseContactInfo(value: unknown): ContactInfo | null {
    if (typeof value !== 'object' || value === null) {
        return null;
    }
    const email = readString(value, 'email');
    const emailHref = readString(value, 'emailHref');
    const phone = readString(value, 'phone');
    const phoneHref = readString(value, 'phoneHref');
    const zaloHref = readString(value, 'zaloHref');
    const zaloQrUrl = readString(value, 'zaloQrUrl');
    const zaloDisplayName = readString(value, 'zaloDisplayName');
    if (
        !email ||
        !emailHref ||
        !phone ||
        !phoneHref ||
        !zaloHref ||
        !zaloQrUrl ||
        !zaloDisplayName
    ) {
        return null;
    }
    return {
        email,
        emailHref,
        phone,
        phoneHref,
        zaloHref,
        zaloQrUrl,
        zaloDisplayName,
    };
}
