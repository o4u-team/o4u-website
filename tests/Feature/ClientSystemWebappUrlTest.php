<?php

namespace Tests\Feature;

use App\Models\App;
use App\Models\Client;
use App\Models\ClientSystem;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class ClientSystemWebappUrlTest extends TestCase
{
    use RefreshDatabase;

    public function test_ensure_and_info_resolve_url_for_each_app_and_fall_back(): void
    {
        $app = App::create(['name' => 'One', 'status' => 'active', 'webapp_url' => 'https://default.example.com']);
        $other = App::create(['name' => 'Two', 'status' => 'active', 'webapp_url' => 'https://other.example.com']);
        $payload = ['endpoint' => 'https://odoo.example.com', 'db_name' => 'tenant'];
        $this->postJson('/api/app/client-systems/ensure', $payload, ['X-App-Id' => $app->uuid])
            ->assertOk()->assertJsonPath('data.webapp_url', $app->webapp_url);
        $system = ClientSystem::firstOrFail();
        $system->update(['allow_get_info' => true]);
        $system->apps()->updateExistingPivot($app->id, ['webapp_url' => 'https://custom.example.com']);

        foreach ([$app, $other] as $current) {
            $expected = $current->id === $app->id ? 'https://custom.example.com' : $other->webapp_url;
            $this->postJson('/api/app/client-systems/ensure', $payload, ['X-App-Id' => $current->uuid])
                ->assertOk()->assertJsonPath('data.webapp_url', $expected);
            $this->getJson('/api/app-info', ['X-App-Id' => $current->uuid, 'X-Client-System-Id' => $system->uuid])
                ->assertOk()->assertJsonPath('data.webapp_url', $expected);
        }

        foreach ([null, ''] as $empty) {
            $system->apps()->updateExistingPivot($app->id, ['webapp_url' => $empty]);
            $this->getJson('/api/app-info', ['X-App-Id' => $app->uuid, 'X-Client-System-Id' => $system->uuid])
                ->assertOk()->assertJsonPath('data.webapp_url', $app->webapp_url);
        }
        $app->update(['webapp_url' => null]);
        $this->postJson('/api/app/client-systems/ensure', $payload, ['X-App-Id' => $app->uuid])
            ->assertOk()->assertJsonPath('data.webapp_url', null);
    }

    public function test_version_settings_apply_to_ensure_info_and_version_api(): void
    {
        $app = App::create(['name' => 'App', 'status' => 'active',
            'android_min_version' => '1.0.0', 'android_current_version' => '3.0.0',
            'ios_current_version' => '4.0.0', 'apple_store_url' => 'https://apps.apple.com/default']);
        $payload = ['endpoint' => 'https://odoo.example.com', 'db_name' => 'tenant'];
        $this->postJson('/api/app/client-systems/ensure', $payload, ['X-App-Id' => $app->uuid])->assertOk();
        $system = ClientSystem::firstOrFail();
        $system->update(['allow_get_info' => true]);
        $overrides = ['android_min_version' => '2.0.0', 'android_store_url' => 'https://play.google.com/custom',
            'ios_min_version' => '2.1.0'];
        $system->apps()->updateExistingPivot($app->id, $overrides);
        $expected = array_merge($overrides, ['android_current_version' => '3.0.0',
            'ios_current_version' => '4.0.0', 'apple_store_url' => 'https://apps.apple.com/default']);
        $headers = ['X-App-Id' => $app->uuid, 'X-Client-System-Id' => $system->uuid];
        $this->postJson('/api/app/client-systems/ensure', $payload, $headers)->assertOk()->assertJson(['data' => $expected]);
        $this->getJson('/api/app-info', $headers)->assertOk()->assertJson(['data' => $expected]);
        $this->getJson('/api/app/version?platform=android&app_version=1.5.0', $headers)
            ->assertOk()->assertJsonPath('data.min_version', '2.0.0')->assertJsonPath('data.force_update', true);
        $this->getJson('/api/app/version?platform=android&app_version=1.5.0', ['X-App-Id' => $app->uuid])
            ->assertOk()->assertJsonPath('data.min_version', '1.0.0')->assertJsonPath('data.force_update', false);
        $unlinked = App::create(['name' => 'Unlinked', 'status' => 'active']);
        $this->getJson('/api/app/version', ['X-App-Id' => $unlinked->uuid, 'X-Client-System-Id' => $system->uuid])->assertForbidden();
    }

    public function test_admin_can_save_validate_preserve_and_clear_app_url(): void
    {
        $this->actingAs(User::factory()->create());
        $client = Client::create(['name' => 'Client', 'status' => 'active']);
        $system = ClientSystem::create(['name' => 'System', 'client_id' => $client->id, 'endpoint' => 'https://odoo.example.com', 'db_name' => 'tenant', 'status' => 'active']);
        $app = App::create(['name' => 'App', 'status' => 'active']);
        $url = "/client-systems/{$system->id}/sync-apps";
        $this->post($url, ['app_ids' => [$app->id], 'webapp_urls' => [$app->id => 'https://custom.example.com']])->assertSessionHasNoErrors();
        $this->getJson("/client-systems/{$system->id}/apps")
            ->assertOk()->assertJsonPath('apps.0.webapp_url', 'https://custom.example.com');
        $this->post($url, ['app_ids' => [$app->id]])->assertSessionHasNoErrors();
        $this->assertSame('https://custom.example.com', $system->webappUrlFor($app));
        $this->post($url, ['app_ids' => [$app->id], 'webapp_urls' => [$app->id => 'invalid']])
            ->assertSessionHasErrors("webapp_urls.{$app->id}");
        $this->post($url, ['app_ids' => [$app->id], 'webapp_urls' => [$app->id => null]])->assertSessionHasNoErrors();
        $this->assertNull($system->webappUrlFor($app));
        $this->post($url, ['app_ids' => [$app->id], 'app_settings' => [$app->id => [
            'android_min_version' => '2.0.0', 'android_current_version' => '1.0.0',
        ]]])->assertSessionHasErrors("app_settings.{$app->id}.android_min_version");
        $this->post($url, ['app_ids' => [$app->id], 'app_settings' => [$app->id => [
            'ios_min_version' => 'bad', 'apple_store_url' => 'bad',
        ]]])->assertSessionHasErrors(["app_settings.{$app->id}.ios_min_version", "app_settings.{$app->id}.apple_store_url"]);
        $this->post($url, ['app_ids' => [$app->id], 'app_settings' => [$app->id => [
            'ios_min_version' => '1.0.0', 'ios_current_version' => '2.0.0', 'apple_store_url' => 'https://apps.apple.com/custom',
        ]]])->assertSessionHasNoErrors();
        $this->assertSame('1.0.0', $system->appSettingsFor($app)['ios_min_version']);
    }
}
