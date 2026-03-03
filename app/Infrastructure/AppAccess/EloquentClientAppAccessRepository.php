<?php

declare(strict_types=1);

namespace App\Infrastructure\AppAccess;

use App\Domain\AppAccess\ClientAppAccessRepository;
use App\Models\ClientSystem;

final readonly class EloquentClientAppAccessRepository implements ClientAppAccessRepository
{
    public function findClientSystemUuidForApp(
        int $appId,
        string $clientDomain,
        string $dbName,
    ): ?string {
        /** @var ClientSystem|null $clientSystem */
        $clientSystem = ClientSystem::query()
            ->where('endpoint', $clientDomain)
            ->where('db_name', $dbName)
            ->where('status', 'active')
            ->whereHas('apps', static function ($q) use ($appId): void {
                $q->where('apps.id', $appId)->where('apps.status', 'active');
            })
            ->first();

        return $clientSystem?->uuid;
    }
}

