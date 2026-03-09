<?php

namespace App\Http\Middleware;

use App\Models\App;
use Closure;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Symfony\Component\HttpFoundation\Response;

class O4uAppMiddleware
{
    /**
     * Handle an incoming request.
     *
     * @param  \Closure(\Illuminate\Http\Request): (\Symfony\Component\HttpFoundation\Response)  $next
     */
    public function handle(Request $request, Closure $next): Response
    {
        if (!$request->header('X-App-Id')) {
            Log::info(__METHOD__, [
                'message' => 'Missing app id',
                'ip' => $request->ip(),
                'header' => $request->header(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Forbiden',
            ], JsonResponse::HTTP_FORBIDDEN);
        }

        try {
            $app = App::where('uuid', $request->header('X-App-Id'))
                ->where('status', 'active')
                ->first();
        } catch (\Throwable $e) {
            Log::error(__METHOD__ . ': Middleware error', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Internal server error.',
            ], JsonResponse::HTTP_BAD_REQUEST);
        }

        if (!$app) {
            Log::info(__METHOD__, [
                'message' => 'Invalid app id',
                'data' => $request->header('X-App-Id'),
                'ip' => $request->ip(),
                'header' => $request->header(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Invalid credentials',
            ], JsonResponse::HTTP_BAD_REQUEST);
        }

        $request->merge([
            'app' => $app,
        ]);

        return $next($request);
    }
}
