<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AppInfoController extends Controller
{
    /**
     * Trả về thông tin endpoint Odoo, webapp_endpoint và db_name
     * dựa trên client_system đã được middleware ValidateClientSystemAppMiddleware
     * gắn vào request.
     */
    public function show(Request $request): JsonResponse
    {
        $clientSystem = $request->get('client_system');

        if (!$clientSystem) {
            return response()->json([
                'success' => false,
                'message' => 'Client system not found in request.',
            ], 400);
        }

        if (!$clientSystem->allow_get_info) {
            return response()->json([
                'success' => false,
                'message' => 'Client system does not allow getting info.',
            ], 403);
        }

        return response()->json([
            'success' => true,
            'data' => [
                'endpoint' => $clientSystem->endpoint,
                'webapp_endpoint' => $clientSystem->webapp_endpoint,
                'db_name' => $clientSystem->db_name,
            ],
        ]);
    }
}

