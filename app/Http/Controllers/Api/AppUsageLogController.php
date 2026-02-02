<?php

namespace App\Http\Controllers\Api;

use App\Models\AppConnectionLog;
use App\Models\AppUserDevice;
use App\Models\UserDevice;
use App\Http\Controllers\Controller;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AppUsageLogController extends Controller
{
    /**
     * Log app usage from mobile app.
     * Requires ValidateClientSystemAppMiddleware (X-App-Id, X-Client-System-Id).
     * $request->app and $request->client_system are set by middleware.
     */
    public function log(Request $request): JsonResponse
    {
        $app = $request->get('app');
        $clientSystem = $request->get('client_system');

        if (!$app || !$clientSystem) {
            return response()->json([
                'success' => false,
                'message' => 'Invalid request. Apply ValidateClientSystemAppMiddleware.',
            ], 403);
        }

        $validated = $request->validate([
            'username' => 'required|string|max:255',
            'device_id' => 'required|string|max:255',
            'device_info' => 'nullable|array',
            'device_info.platform' => 'nullable|string|max:100',
            'device_info.model' => 'nullable|string|max:255',
            'device_info.os_version' => 'nullable|string|max:50',
            'device_info.app_version' => 'nullable|string|max:50',
        ]);

        $username = $validated['username'];
        $deviceId = $validated['device_id'];
        $deviceInfo = $validated['device_info'] ?? null;
        $ipAddress = $request->ip();
        $appVersion = $deviceInfo['app_version'] ?? $request->input('app_version');

        // 1. Find or create user_device (only device info)
        $userDevice = UserDevice::firstOrCreate(
            ['device_id' => $deviceId],
            ['device_info' => $deviceInfo]
        );
        $userDevice->update([
            'device_info' => $deviceInfo,
            'last_connected_at' => now(),
        ]);

        // 2. Find or create app_user_device (app + client_system + device + username)
        $appUserDevice = AppUserDevice::firstOrCreate(
            [
                'app_id' => $app->id,
                'client_system_id' => $clientSystem->id,
                'user_device_id' => $userDevice->id,
            ],
            ['username' => $username]
        );
        $appUserDevice->update([
            'username' => $username,
            'last_connected_at' => now(),
            'app_version' => $appVersion,
        ]);

        // 3. Create connection log (history) - version at each call
        AppConnectionLog::create([
            'app_user_device_id' => $appUserDevice->id,
            'app_version' => $appVersion,
            'ip_address' => $ipAddress,
        ]);

        $clientSystem->load('client:id,name,uuid');

        return response()->json([
            'success' => true,
            'message' => 'Logged successfully.',
            'client_system' => [
                'id' => $clientSystem->id,
                'name' => $clientSystem->name,
                'uuid' => $clientSystem->uuid,
                'endpoint' => $clientSystem->endpoint,
                'db_name' => $clientSystem->db_name,
                'status' => $clientSystem->status,
                'client' => $clientSystem->client ? [
                    'id' => $clientSystem->client->id,
                    'name' => $clientSystem->client->name,
                    'uuid' => $clientSystem->client->uuid,
                ] : null,
            ],
        ]);
    }
}
