<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    public function up(): void
    {
        Schema::table('app_client_system', function (Blueprint $table) {
            $table->string('webapp_url')->nullable();
        });
    }

    public function down(): void
    {
        Schema::table('app_client_system', function (Blueprint $table) {
            $table->dropColumn('webapp_url');
        });
    }
};
