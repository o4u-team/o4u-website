<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Http\Support\ApiRequestHeaders;
use App\Models\App;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AppVersionController extends Controller
{
    /**
     * Return minimum/current app versions for force-update checks.
     * Requires header: X-App-Id (app uuid), resolved by O4uAppMiddleware.
     *
     * Optional:
     * - X-Platform or ?platform=android|ios
     * - X-App-Version or ?app_version=1.0.0 (client version for force_update flag)
     */
    public function show(Request $request): JsonResponse
    {
        /** @var App|null $app */
        $app = $request->get('app');

        if (!$app instanceof App) {
            return response()->json([
                'success' => false,
                'message' => 'App not found in request.',
            ], 400);
        }

        $platform = strtolower((string) (ApiRequestHeaders::value($request, 'X-Platform') ?? $request->query('platform', '')));
        $clientVersion = ApiRequestHeaders::value($request, 'X-App-Version') ?? ApiRequestHeaders::normalize($request->query('app_version'));

        if ($platform !== '') {
            if (!in_array($platform, ['android', 'ios'], true)) {
                return response()->json([
                    'success' => false,
                    'message' => 'Invalid platform. Use android or ios.',
                ], 422);
            }

            return response()->json([
                'success' => true,
                'data' => $this->buildPlatformVersionData($app, $platform, $clientVersion),
            ]);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'android' => $this->buildPlatformVersionData($app, 'android', $clientVersion),
                'ios' => $this->buildPlatformVersionData($app, 'ios', $clientVersion),
            ],
        ]);
    }

    /**
     * @return array{
     *     platform: string,
     *     min_version: string|null,
     *     current_version: string|null,
     *     store_url: string|null,
     *     force_update: bool
     * }
     */
    private function buildPlatformVersionData(App $app, string $platform, mixed $clientVersion): array
    {
        $isAndroid = $platform === 'android';

        $minVersion = $isAndroid ? $app->android_min_version : $app->ios_min_version;
        $currentVersion = $isAndroid ? $app->android_current_version : $app->ios_current_version;
        $storeUrl = $isAndroid ? $app->android_store_url : $app->apple_store_url;

        return [
            'platform' => $platform,
            'min_version' => $minVersion,
            'current_version' => $currentVersion,
            'store_url' => $storeUrl,
            'force_update' => $this->shouldForceUpdate($clientVersion, $minVersion),
        ];
    }

    private function shouldForceUpdate(mixed $clientVersion, ?string $minVersion): bool
    {
        if (!is_string($clientVersion) || $clientVersion === '' || empty($minVersion)) {
            return false;
        }

        return App::compareVersions($clientVersion, $minVersion) < 0;
    }
}
