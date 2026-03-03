<?php

declare(strict_types=1);

namespace App\Application\AppAccess\Query;

final readonly class CheckClientAppAccessQuery
{
    public function __construct(
        public int $appId,
        public string $clientDomain,
        public string $dbName,
    ) {
    }
}

