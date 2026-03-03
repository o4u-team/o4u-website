<?php

declare(strict_types=1);

namespace App\Application\AppAccess\Handler;

use App\Application\AppAccess\DTO\CheckClientAppAccessResult;
use App\Application\AppAccess\Query\CheckClientAppAccessQuery;
use App\Domain\AppAccess\ClientAppAccessRepository;

final readonly class CheckClientAppAccessHandler
{
    public function __construct(
        private ClientAppAccessRepository $repository,
    ) {
    }

    public function handle(CheckClientAppAccessQuery $query): CheckClientAppAccessResult
    {
        $uuid = $this->repository->findClientSystemUuidForApp(
            $query->appId,
            $query->clientDomain,
            $query->dbName,
        );

        return new CheckClientAppAccessResult(
            status: $uuid !== null,
            clientSystemUuid: $uuid,
        );
    }
}

