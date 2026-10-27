<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::table('app_client_system', function (Blueprint $table) {
            $table->string('android_min_version', 255)->nullable();
            $table->string('android_current_version', 255)->nullable();
            $table->string('ios_min_version', 255)->nullable();
            $table->string('ios_current_version', 255)->nullable();
            $table->string('android_store_url', 500)->nullable();
            $table->string('apple_store_url', 500)->nullable();
        });
    }

    public function down(): void
    {
        Schema::table('app_client_system', function (Blueprint $table) {
            $table->dropColumn(['android_min_version', 'android_current_version', 'ios_min_version', 'ios_current_version', 'android_store_url', 'apple_store_url']);
        });
    }
};
