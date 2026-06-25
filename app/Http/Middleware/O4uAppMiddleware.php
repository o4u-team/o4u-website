<?php

namespace App\Http\Middleware;

use App\Http\Support\ApiRequestHeaders;
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
        $appId = ApiRequestHeaders::value($request, 'X-App-Id');

        if ($appId === null) {
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

        $appUuid = ApiRequestHeaders::uuid($appId);

        if ($appUuid === null) {
            Log::info(__METHOD__, [
                'message' => 'Invalid app id format',
                'data' => $appId,
                'ip' => $request->ip(),
            ]);

            return response()->json([
                'success' => false,
                'message' => 'Invalid credentials',
            ], JsonResponse::HTTP_BAD_REQUEST);
        }

        $app = App::where('uuid', $appUuid)
            ->where('status', 'active')
            ->first();

        if (!$app) {
            Log::info(__METHOD__, [
                'message' => 'Invalid app id',
                'data' => $appUuid,
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
