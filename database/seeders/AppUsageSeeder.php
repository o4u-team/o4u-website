<?php

namespace Database\Seeders;

use App\Models\App;
use App\Models\AppConnectionLog;
use App\Models\AppUserDevice;
use App\Models\Client;
use App\Models\ClientSystem;
use App\Models\UserDevice;
use Illuminate\Database\Seeder;

class AppUsageSeeder extends Seeder
{
    /**
     * Seed app usage test data: user_devices, app_user_devices, app_connection_logs.
     */
    public function run(): void
    {
        // Ensure we have at least one app and client_system linked
        $app = App::where('status', 'active')->first();
        $clientSystem = ClientSystem::where('status', 'active')->first();

        if (!$app || !$clientSystem) {
            $client = Client::firstOrCreate(
                ['name' => 'Client Test'],
                ['status' => 'active']
            );

            $clientSystem = ClientSystem::firstOrCreate(
                [
                    'client_id' => $client->id,
                    'db_name' => 'test_db',
                ],
                [
                    'name' => 'System Test',
                    'endpoint' => 'https://api.test.example.com',
                    'status' => 'active',
                ]
            );

            $app = App::firstOrCreate(
                ['name' => 'App Test'],
                [
                    'status' => 'active',
                    'android_current_version' => '1.0.0',
                    'ios_current_version' => '1.0.0',
                ]
            );

            $clientSystem->apps()->syncWithoutDetaching([$app->id]);
        } else {
            // Ensure app is allowed for this client_system
            if (!$clientSystem->apps()->where('apps.id', $app->id)->exists()) {
                $clientSystem->apps()->attach($app->id);
            }
        }

        // Create user_devices (only device info)
        $userDevices = [];
        $deviceIds = [
            'device-android-001',
            'device-ios-002',
            'device-android-003',
            'device-android-004',
            'device-ios-005',
        ];
        $deviceInfos = [
            ['platform' => 'android', 'model' => 'Samsung Galaxy S21', 'os_version' => '14', 'app_version' => '1.2.0'],
            ['platform' => 'ios', 'model' => 'iPhone 14', 'os_version' => '17', 'app_version' => '1.2.0'],
            ['platform' => 'android', 'model' => 'Pixel 7', 'os_version' => '14', 'app_version' => '1.1.5'],
            ['platform' => 'android', 'model' => 'Xiaomi 13', 'os_version' => '13', 'app_version' => '1.2.0'],
            ['platform' => 'ios', 'model' => 'iPhone 13', 'os_version' => '16', 'app_version' => '1.1.0'],
        ];

        foreach ($deviceIds as $i => $deviceId) {
            $userDevices[] = UserDevice::firstOrCreate(
                ['device_id' => $deviceId],
                [
                    'device_info' => $deviceInfos[$i] ?? null,
                    'last_connected_at' => now()->subMinutes(rand(5, 120)),
                ]
            );
        }

        // Create app_user_devices (app + client_system + device + username + app_version)
        $usernames = ['user1@test.com', 'user2@test.com', 'admin@test.com', 'sale@test.com', 'support@test.com'];
        $appVersions = ['1.2.0', '1.2.0', '1.1.5', '1.2.0', '1.1.0'];
        $appUserDevices = [];

        foreach ($userDevices as $i => $userDevice) {
            $appUserDevices[] = AppUserDevice::firstOrCreate(
                [
                    'app_id' => $app->id,
                    'client_system_id' => $clientSystem->id,
                    'user_device_id' => $userDevice->id,
                ],
                [
                    'username' => $usernames[$i] ?? 'user' . ($i + 1) . '@test.com',
                    'app_version' => $appVersions[$i] ?? '1.0.0',
                    'last_connected_at' => now()->subMinutes(rand(1, 60)),
                ]
            );
        }

        // Update last_connected_at on user_devices
        foreach ($userDevices as $ud) {
            $ud->update(['last_connected_at' => now()->subMinutes(rand(1, 120))]);
        }

        // Create app_connection_logs (history)
        $ips = ['192.168.1.1', '10.0.0.5', '172.16.0.10', '103.45.67.89', '14.231.123.45'];

        $versions = ['1.1.0', '1.1.5', '1.2.0'];
        foreach ($appUserDevices as $appUserDevice) {
            $count = rand(2, 5);
            for ($i = 0; $i < $count; $i++) {
                $log = AppConnectionLog::create([
                    'app_user_device_id' => $appUserDevice->id,
                    'app_version' => $versions[array_rand($versions)],
                    'ip_address' => $ips[array_rand($ips)],
                ]);
                $log->created_at = now()->subDays(rand(0, 7))->subHours(rand(0, 23))->subMinutes(rand(0, 59));
                $log->saveQuietly();
            }
        }
    }
}
