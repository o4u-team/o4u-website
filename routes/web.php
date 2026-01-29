<?php

use App\Http\Controllers\AppController;
use App\Http\Controllers\Auth\SocialiteController;
use App\Http\Controllers\Web\AppUsageController;
use App\Http\Controllers\Web\ClientController;
use App\Http\Controllers\Web\ClientSystemController;
use Illuminate\Support\Facades\Route;

// Public routes
Route::middleware('guest')->group(function () {
    Route::inertia('/login', 'Auth/Login')->name('login');
    Route::get('/auth/google', [SocialiteController::class, 'redirectToGoogle'])->name('auth.google');
    Route::get('/auth/callback', [SocialiteController::class, 'handleGoogleCallback']);
});

// Protected routes
Route::middleware('auth')->group(function () {
    Route::inertia('/', 'Home')->name('home');
    Route::post('/logout', [SocialiteController::class, 'logout'])->name('logout');

    // Clients CRUD
    Route::resource('clients', ClientController::class);
    
    // Client Systems CRUD
    Route::resource('client-systems', ClientSystemController::class);
    Route::get('client-systems/{client_system}/apps', [ClientSystemController::class, 'getApps'])
        ->name('client-systems.apps');
    Route::post('client-systems/{client_system}/sync-apps', [ClientSystemController::class, 'syncApps'])
        ->name('client-systems.sync-apps');
    
    // Apps CRUD
    Route::resource('apps', AppController::class);

    // App Usage (devices + connection logs)
    Route::get('app-usage', [AppUsageController::class, 'index'])->name('app-usage.index');
});

// Legacy route
Route::view('/kho-barcode', 'kho_barcode');
