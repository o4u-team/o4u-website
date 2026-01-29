<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::table('app_user_devices', function (Blueprint $table) {
            $table->string('app_version', 50)->nullable()->after('last_connected_at');
        });

        Schema::table('app_connection_logs', function (Blueprint $table) {
            $table->string('app_version', 50)->nullable()->after('app_user_device_id');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('app_user_devices', function (Blueprint $table) {
            $table->dropColumn('app_version');
        });

        Schema::table('app_connection_logs', function (Blueprint $table) {
            $table->dropColumn('app_version');
        });
    }
};
