<?php

use App\Http\Controllers\Auth\SocialiteController;
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
});

// Legacy route
Route::view('/kho-barcode', 'kho_barcode');
