<?php

namespace App\Http\Support;

use Illuminate\Http\Request;
use Illuminate\Support\Str;

class ApiRequestHeaders
{
    public static function value(Request $request, string $name): ?string
    {
        return self::normalize($request->header($name));
    }

    public static function normalize(mixed $value): ?string
    {
        if (!is_string($value)) {
            return null;
        }

        $normalized = trim($value);

        return $normalized === '' ? null : $normalized;
    }

    public static function uuid(mixed $value): ?string
    {
        $normalized = self::normalize($value);

        if ($normalized === null) {
            return null;
        }

        return Str::isUuid($normalized) ? $normalized : null;
    }
}
