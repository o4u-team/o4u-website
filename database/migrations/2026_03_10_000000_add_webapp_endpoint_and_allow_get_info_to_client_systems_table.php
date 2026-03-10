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
            $table->string('webapp_endpoint')->nullable()->after('endpoint');
            $table->boolean('allow_get_info')->default(false)->after('db_name');
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::table('client_systems', function (Blueprint $table) {
            $table->dropColumn(['webapp_endpoint', 'allow_get_info']);
        });
    }
};

