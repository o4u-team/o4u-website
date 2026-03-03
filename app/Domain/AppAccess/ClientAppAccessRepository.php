<?php

declare(strict_types=1);

namespace App\Domain\AppAccess;

interface ClientAppAccessRepository
{
    /**
     * Return client_system UUID if the given app is allowed for the
     * client_system identified by (endpoint, db_name) and both are active.
     */
    public function findClientSystemUuidForApp(
        int $appId,
        string $clientDomain,
        string $dbName,
    ): ?string;
}

