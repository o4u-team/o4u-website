<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Application\ClientSystem\EnsureClientSystem;
use App\Http\Controllers\Controller;
use App\Http\Support\ApiRequestHeaders;
use App\Models\App;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class EnsureClientSystemController extends Controller
{
    public function __construct(
        private readonly EnsureClientSystem $ensureClientSystem,
    ) {
    }

    public function store(Request $request): JsonResponse
    {
        $data = $request->validate([
            'endpoint' => ['required', 'string', 'max:255', 'url:http,https'],
            'db_name' => ['required', 'string', 'max:255'],
        ]);

        $appUuid = ApiRequestHeaders::uuid(ApiRequestHeaders::value($request, 'X-App-Id'));
        $app = $appUuid === null
            ? null
            : App::query()->where('uuid', $appUuid)->where('status', 'active')->first();
        if (!$app instanceof App || !is_numeric($app->getKey()) || (int) $app->getKey() < 1) {
            return response()->json([
                'success' => false,
                'message' => 'Invalid app.',
            ], 400);
        }

        $result = $this->ensureClientSystem->handle(
            $app,
            $data['endpoint'],
            $data['db_name'],
        );

        return response()->json([
            'success' => true,
            'data' => $result,
        ]);
    }
}
