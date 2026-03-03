<?php

declare(strict_types=1);

namespace App\Application\AppAccess\DTO;

final readonly class CheckClientAppAccessResult
{
    public function __construct(
        public bool $status,
        public ?string $clientSystemUuid,
    ) {
    }
}

