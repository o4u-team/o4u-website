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
        Schema::create('app_user_devices', function (Blueprint $table) {
            $table->id();
            $table->foreignId('app_id')->constrained('apps')->onDelete('cascade');
            $table->foreignId('client_system_id')->constrained('client_systems')->onDelete('cascade');
            $table->foreignId('user_device_id')->constrained('user_devices')->onDelete('cascade');
            $table->string('username');
            $table->timestamp('last_connected_at')->nullable();
            $table->timestamps();

            // One device per app per client_system (same device can be linked once per app+client_system)
            $table->unique(['app_id', 'client_system_id', 'user_device_id']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('app_user_devices');
    }
};
