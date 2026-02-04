<?php

namespace App\Http\Controllers\Api;

use App\Http\Controllers\Controller;
use App\Jobs\FCMSendMultiNotiJob;
use App\Jobs\FCMSendNotiJob;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Throwable;

class FcmController extends Controller
{
    public function __construct()
    {
    }

    public function sendNoti(Request $request): JsonResponse
    {
        Log::info(__METHOD__, [
            'message' => '==== FCM CONTROLLER====',
        ]);
        $request->validate([
            'fcm_token' => 'required|string',
            'data' => 'nullable|array',
            'title' => 'required|string',
            'description' => 'required|string',
        ]);

        Log::info(__METHOD__, [
            'message' => '==== Request====',
            'data' => $request->all(),
        ]);

        try {
            $title = $request->title;
            $description = $request->description;
            $data = $request->data ?? [];
            $token = $request->fcm_token;

            FCMSendNotiJob::dispatch($title, $description, $token, $data);

            return response()->json([
                'success' => true,
            ]);
        } catch (Throwable $e) {
            Log::info(__METHOD__, [
                'message' => '==== Error ====',
                'data' => $e,
            ]);

            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ]);
        }
    }

    /**
     * Send notification to multiple device tokens at once (multicast).
     * Body: fcm_tokens (array), title, description, data (optional).
     * Max 500 tokens per request (Firebase FCM limit).
     */
    public function sendMultiNoti(Request $request): JsonResponse
    {
        $request->validate([
            'fcm_tokens' => 'required|array',
            'fcm_tokens.*' => 'required|string',
            'title' => 'required|string|max:255',
            'description' => 'required|string',
            'data' => 'nullable|array',
        ]);

        $tokens = array_values(array_unique($request->fcm_tokens));

        if (count($tokens) > 500) {
            return response()->json([
                'success' => false,
                'error' => 'Maximum 500 device tokens per request.',
            ], 422);
        }

        if (empty($tokens)) {
            return response()->json([
                'success' => false,
                'error' => 'At least one FCM token is required.',
            ], 422);
        }

        try {
            FCMSendMultiNotiJob::dispatch(
                $request->title,
                $request->description,
                $tokens,
                $request->data ?? []
            );

            return response()->json([
                'success' => true,
                'message' => 'Notification queued for ' . count($tokens) . ' device(s).',
                'token_count' => count($tokens),
            ]);
        } catch (Throwable $e) {
            Log::error(__METHOD__ . ': send-multi error', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);

            return response()->json([
                'success' => false,
                'error' => $e->getMessage(),
            ], 500);
        }
    }
}
