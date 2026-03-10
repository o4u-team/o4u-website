<?php

use App\Http\Controllers\Api\AppInfoController;
use App\Http\Controllers\Api\AppPermissionController;
use App\Http\Controllers\Api\AppUsageLogController;
use App\Http\Controllers\Api\DataCryptoController;
use App\Http\Controllers\Api\FcmController;
use App\Http\Controllers\Api\UserController;
use App\Http\Middleware\O4uApiKeyMiddleware;
use App\Http\Middleware\O4uAppMiddleware;
use App\Http\Middleware\ValidateClientSystemAppMiddleware;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Route;

Route::post('odoo-barcode/log', function(Request $request) {
    $data = $request->all();

    Log::channel('odoo_barcode')->info(json_encode($data));
});

Route::post('odoo-approval/log', function (Request $request) {
    $data = $request->all();

    Log::channel('odoo_approval')->info(json_encode($data));
});

Route::middleware([
    O4uAppMiddleware::class,
])->group(function() {
    Route::post('/app/permissions', [AppPermissionController::class, 'checkPermissions']);
    Route::post('/app/check-access', [AppPermissionController::class, 'checkClientAppAccess']);
    Route::post('/data/decrypt', [DataCryptoController::class, 'decrypt']);
    Route::post('/user/device', [UserController::class, 'updateDeviceInfo']);
});

// Trả về thông tin client_system (endpoint, webapp_endpoint, db_name)
// cần headers: X-App-Id, X-Client-System-Id
Route::middleware([
    ValidateClientSystemAppMiddleware::class,
])->get('/app-info', [AppInfoController::class, 'show']);

Route::middleware([
    O4uApiKeyMiddleware::class
])->group(function() {
    Route::post('/data/encrypt', [DataCryptoController::class, 'encrypt']);

    Route::post('fcm/send', [FcmController::class, 'sendNoti']);
});

Route::middleware([
    ValidateClientSystemAppMiddleware::class
])->prefix('v1')->name('v1.')->group(function() {
    // Firebase FCM
    Route::post('fcm/send-multi', [FcmController::class, 'sendMultiNoti'])->name('fcm.send-multi');

    Route::post('app-usage/log', [AppUsageLogController::class, 'log'])->name('app-usage.log');
});
