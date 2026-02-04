<?php

namespace App\Http\Middleware;

use App\Models\App;
use App\Models\ClientSystem;
use Closure;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Symfony\Component\HttpFoundation\Response;

class ValidateClientSystemAppMiddleware
{
    /**
     * Validate that client_system (by uuid) and app (by uuid or id) are linked.
     * Expects headers: X-Client-System-Id (client_system uuid), X-App-Id (app uuid).
     * Merges validated $clientSystem and $app into request for use in controller.
     *
     * @param  \Closure(\Illuminate\Http\Request): (\Symfony\Component\HttpFoundation\Response)  $next
     */
    public function handle(Request $request, Closure $next): Response
    {
        try {
            $clientSystemId = $request->header('X-Client-System-Id');
            $appId = $request->header('X-App-Id');

            if (empty($clientSystemId) || empty($appId)) {
                return response()->json([
                    'success' => false,
                    'message' => 'Missing required headers: X-Client-System-Id, X-App-Id',
                ], 400);
            }

            $clientSystem = ClientSystem::where('uuid', $clientSystemId)
                ->where('status', 'active')
                ->first();

            if (!$clientSystem) {
                return response()->json([
                    'success' => false,
                    'message' => 'Invalid or inactive client system.',
                ], 404);
            }

            // Accept app by uuid (preferred) or numeric id
            $app = is_numeric($appId)
                ? App::where('id', (int) $appId)->where('status', 'active')->first()
                : App::where('uuid', $appId)->where('status', 'active')->first();

            if (!$app) {
                return response()->json([
                    'success' => false,
                    'message' => 'Invalid or inactive app.',
                ], 404);
            }

            if (!$clientSystem->apps()->where('apps.id', $app->id)->exists()) {
                return response()->json([
                    'success' => false,
                    'message' => 'App is not allowed for this client system.',
                ], 403);
            }

            $request->merge([
                'client_system' => $clientSystem,
                'app' => $app,
            ]);

            return $next($request);
        } catch (\Throwable $e) {
            Log::error(__METHOD__ . ': Middleware error', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Internal server error.',
            ], 500);
        }
    }
}
