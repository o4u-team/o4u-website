<?php

declare(strict_types=1);

namespace App\Application\ClientSystem;

use App\Models\App;
use App\Models\Client;
use App\Models\ClientSystem;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\DB;

class EnsureClientSystem
{
    /**
     * Find the client system for this Odoo endpoint and database, or create it
     * and grant the calling app.
     *
     * @return array{client_system_id: string, created: bool}
     */
    public function handle(App $app, string $endpoint, string $dbName): array
    {
        $endpoint = $this->normalizeEndpoint($endpoint);
        $dbName = trim($dbName);

        return DB::transaction(function () use ($app, $endpoint, $dbName): array {
            $existing = $this->find($endpoint, $dbName);
            if ($existing instanceof ClientSystem) {
                if ($existing->status !== 'active') {
                    $existing->status = 'active';
                    $existing->save();
                }
                $existing->apps()->syncWithoutDetaching([$app->id]);

                return [
                    'client_system_id' => (string) $existing->uuid,
                    'created' => false,
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
                'client_id' => $client->id,
                'endpoint' => $endpoint,
                'db_name' => $dbName,
                'status' => 'active',
                'allow_get_info' => false,
            ]);
            $system->apps()->syncWithoutDetaching([$app->id]);

            return [
                'client_system_id' => (string) $system->uuid,
                'created' => true,
            ];
        });
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
