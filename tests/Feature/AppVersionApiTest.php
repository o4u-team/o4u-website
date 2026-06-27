<?php

namespace Tests\Feature;

use App\Models\App;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class AppVersionApiTest extends TestCase
{
    use RefreshDatabase;

    public function test_returns_version_info_for_android_platform(): void
    {
        $app = App::create([
            'name' => 'Test App',
            'uuid' => '11111111-1111-1111-1111-111111111111',
            'android_min_version' => '1.2.0',
            'android_current_version' => '1.3.0',
            'android_store_url' => 'https://play.google.com/store/apps/details?id=test',
            'webapp_url' => 'https://webapp.example.com',
            'status' => 'active',
        ]);

        $response = $this->getJson('/api/app/version?platform=android&app_version=1.1.0', [
            'X-App-Id' => $app->uuid,
        ]);

        $response
            ->assertOk()
            ->assertJson([
                'success' => true,
                'data' => [
                    'platform' => 'android',
                    'min_version' => '1.2.0',
                    'current_version' => '1.3.0',
                    'store_url' => 'https://play.google.com/store/apps/details?id=test',
                    'webapp_url' => 'https://webapp.example.com',
                    'force_update' => true,
                ],
            ]);
    }

    public function test_accepts_app_id_header_with_leading_whitespace(): void
    {
        $app = App::create([
            'name' => 'Test App',
            'uuid' => '22222222-2222-2222-2222-222222222222',
            'android_min_version' => '1.0.0',
            'status' => 'active',
        ]);

        $response = $this->getJson('/api/app/version?platform=android', [
            'X-App-Id' => "\t{$app->uuid}",
        ]);

        $response
            ->assertOk()
            ->assertJsonPath('data.min_version', '1.0.0');
    }

    public function test_returns_forbidden_when_app_id_header_is_missing(): void
    {
        $response = $this->getJson('/api/app/version');

        $response
            ->assertForbidden()
            ->assertJson([
                'success' => false,
            ]);
    }
}
