<?php

declare(strict_types=1);

namespace App\Application\ClientSystem;

use App\Models\App;
use App\Models\Client;
use App\Models\ClientSystem;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;
use RuntimeException;

class EnsureClientSystem
{
    /**
     * Find the client system for this Odoo endpoint and database, or create it
     * and grant the calling app.
     *
     * @return array<string, mixed>
     */
    public function handle(App $app, string $endpoint, string $dbName): array
    {
        $endpoint = $this->normalizeEndpoint($endpoint);
        $dbName = trim($dbName);

        $appId = $this->savedId($app);

        return DB::transaction(function () use ($app, $appId, $endpoint, $dbName): array {
            $existing = $this->find($endpoint, $dbName);
            if ($existing instanceof ClientSystem) {
                if ($existing->status !== 'active') {
                    $existing->status = 'active';
                    $existing->save();
                }
                $this->grant($existing, $appId);

                return [
                    'client_system_id' => (string) $existing->uuid,
                    'created' => false,
                    ...$existing->appSettingsFor($app),
                ];
            }

            $host = parse_url($endpoint, PHP_URL_HOST);
            $name = is_string($host) && $host !== '' ? $host : $dbName;
            $client = Client::create([
                'name' => $name,
                'status' => 'active',
            ]);
            $system = ClientSystem::create([
                'name' => $name.' / '.$dbName,
                'client_id' => $this->savedId($client),
                'endpoint' => $endpoint,
                'db_name' => $dbName,
                'status' => 'active',
                'allow_get_info' => false,
            ]);
            $this->grant($system, $appId);

            return [
                'client_system_id' => (string) $system->uuid,
                'created' => true,
                ...$system->appSettingsFor($app),
            ];
        });
    }

    private function grant(ClientSystem $system, int $appId): void
    {
        $systemId = $this->savedId($system);

        DB::table('app_client_system')->insertOrIgnore([
            'app_id' => $appId,
            'client_system_id' => $systemId,
            'created_at' => now(),
            'updated_at' => now(),
        ]);
    }

    private function savedId(Model $model): int
    {
        $id = $model->getKey();
        if (is_numeric($id) && (int) $id > 0) {
            return (int) $id;
        }

        $uuid = $model->getAttribute('uuid');
        if (is_string($uuid) && $uuid !== '') {
            $id = $model->newQuery()->where('uuid', $uuid)->value('id');
        }

        if (!is_numeric($id) || (int) $id < 1) {
            throw new RuntimeException('Saved record is missing an id.');
        }

        return (int) $id;
    }

    private function find(string $endpoint, string $dbName): ?ClientSystem
    {
        /** @var Collection<int, ClientSystem> $candidates */
        $candidates = ClientSystem::query()
            ->where('db_name', $dbName)
            ->get()
            ->filter(function (ClientSystem $system) use ($endpoint): bool {
                return strcasecmp($this->normalizeEndpoint((string) $system->endpoint), $endpoint) === 0;
            })
            ->values();

        $match = $candidates->first();

        return $match instanceof ClientSystem ? $match : null;
    }

    private function normalizeEndpoint(string $endpoint): string
    {
        return rtrim(trim($endpoint), '/');
    }
}
