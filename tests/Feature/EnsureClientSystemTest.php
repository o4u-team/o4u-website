<?php

namespace Tests\Feature;

use App\Models\App;
use App\Models\ClientSystem;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class EnsureClientSystemTest extends TestCase
{
    use RefreshDatabase;

    public function test_creates_client_system_and_links_app(): void
    {
        $app = App::create([
            'name' => 'Approval',
            'uuid' => '11111111-1111-1111-1111-111111111111',
            'status' => 'active',
        ]);

        $response = $this->postJson('/api/app/client-systems/ensure', [
            'endpoint' => 'https://odoo.example.com/',
            'db_name' => 'odoo_approve',
        ], [
            'X-App-Id' => $app->uuid,
        ]);

        $response->assertOk()->assertJsonPath('success', true)->assertJsonPath('data.created', true);

        $system = ClientSystem::query()->where('db_name', 'odoo_approve')->first();
        $this->assertNotNull($system);
        $this->assertSame('https://odoo.example.com', $system->endpoint);
        $this->assertTrue($system->apps()->where('apps.id', $app->id)->exists());
        $response->assertJsonPath('data.client_system_id', $system->uuid);
    }

    public function test_reuses_existing_client_system(): void
    {
        $app = App::create([
            'name' => 'Approval',
            'uuid' => '22222222-2222-2222-2222-222222222222',
            'status' => 'active',
        ]);

        $this->postJson('/api/app/client-systems/ensure', [
            'endpoint' => 'https://odoo.example.com',
            'db_name' => 'odoo_approve',
        ], [
            'X-App-Id' => $app->uuid,
        ])->assertOk();

        $second = $this->postJson('/api/app/client-systems/ensure', [
            'endpoint' => 'https://odoo.example.com/',
            'db_name' => 'odoo_approve',
        ], [
            'X-App-Id' => $app->uuid,
        ]);

        $second->assertOk()->assertJsonPath('data.created', false);
        $this->assertSame(1, ClientSystem::query()->count());
    }

    public function test_rejects_missing_app_header(): void
    {
        $this->postJson('/api/app/client-systems/ensure', [
            'endpoint' => 'https://odoo.example.com',
            'db_name' => 'odoo_approve',
        ])->assertForbidden();
    }
}
