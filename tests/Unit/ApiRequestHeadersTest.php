<?php

namespace Tests\Unit;

use App\Http\Support\ApiRequestHeaders;
use PHPUnit\Framework\TestCase;

class ApiRequestHeadersTest extends TestCase
{
    public function test_normalize_trims_whitespace_and_tabs(): void
    {
        $this->assertSame(
            '54ebdfb9-29db-4407-b0a5-ee5274301694',
            ApiRequestHeaders::normalize("\t54ebdfb9-29db-4407-b0a5-ee5274301694")
        );
    }

    public function test_uuid_accepts_trimmed_uuid(): void
    {
        $this->assertSame(
            '54ebdfb9-29db-4407-b0a5-ee5274301694',
            ApiRequestHeaders::uuid("\t54ebdfb9-29db-4407-b0a5-ee5274301694")
        );
    }

    public function test_uuid_rejects_invalid_value(): void
    {
        $this->assertNull(ApiRequestHeaders::uuid('not-a-uuid'));
    }
}
