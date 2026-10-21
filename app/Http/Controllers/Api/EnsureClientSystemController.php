<?php

declare(strict_types=1);

namespace App\Http\Controllers\Api;

use App\Application\ClientSystem\EnsureClientSystem;
use App\Http\Controllers\Controller;
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

        $app = $request->get('app');
        if (!$app instanceof App) {
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
