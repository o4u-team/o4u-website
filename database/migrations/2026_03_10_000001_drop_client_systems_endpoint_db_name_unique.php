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
        Schema::table('client_systems', function (Blueprint $table) {
            // Bỏ unique constraint trên cặp (endpoint, db_name)
            // Tên index mặc định: client_systems_endpoint_db_name_unique
            $table->dropUnique(['endpoint', 'db_name']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('client_systems', function (Blueprint $table) {
            $table->unique(['endpoint', 'db_name']);
        });
    }
};

