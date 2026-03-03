<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Application\AppAccess\DTO\CheckClientAppAccessResult;
use App\Application\AppAccess\Handler\CheckClientAppAccessHandler;
use App\Application\AppAccess\Query\CheckClientAppAccessQuery;
use App\Http\Controllers\Controller;
use App\Models\App;
use App\Models\ModuleApp;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AppPermissionController extends Controller
{
    public function __construct(
        private readonly CheckClientAppAccessHandler $checkClientAppAccessHandler,
    ) {
    }

    public function checkPermissions(Request $request): JsonResponse
    {
        $modules = $request->get('modules', []);

        $moduleApps = ModuleApp::with(['features'])->get();

        $result = [];

        foreach ($moduleApps as $app) {
            if (in_array($app->name, $modules, true)) {
                $permissions = [];

                foreach ($app->features as $feature) {
                    $permissions[$feature->name] = true;
                }

                $result[$app->name] = $permissions;
                continue;
            }

            $result[$app->name] = false;
        }

        return response()->json([
            'success' => true,
            'data' => $result,
        ]);
    }

    /**
     * Check if a client has permission to use an app via a client_system.
     * Body JSON: { "client_domain": string, "db_name": string }
     * - app: lấy từ $request (đã được merge trong O4uAppMiddleware)
     *
     * Response:
     *  - { "status": true, "client_system_uuid": "..." } if allowed
     *  - { "status": false } otherwise
     */
    public function checkClientAppAccess(Request $request): JsonResponse
    {
        $data = $request->validate([
            'client_domain' => 'required|string',
            'db_name' => 'required|string',
        ]);

        /** @var App|null $app */
        $app = $request->get('app');
        if (!$app instanceof App) {
            return response()->json([
                'status' => false,
            ]);
        }

        $query = new CheckClientAppAccessQuery(
            appId: $app->id,
            clientDomain: $data['client_domain'],
            dbName: $data['db_name'],
        );

        /** @var CheckClientAppAccessResult $result */
        $result = $this->checkClientAppAccessHandler->handle($query);

        if (!$result->status) {
            return response()->json([
                'status' => false,
            ]);
        }

        return response()->json([
            'status' => true,
            'client_system_uuid' => $result->clientSystemUuid,
        ]);
    }
}
