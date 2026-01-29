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
        Schema::create('client_systems', function (Blueprint $table) {
            $table->id();
            $table->string('name');
            $table->uuid('uuid')->unique();
            $table->foreignId('client_id')->constrained('clients')->onDelete('cascade');
            $table->string('endpoint');
            $table->string('db_name');
            $table->enum('status', ['active', 'inactive', 'expired'])->default('active');
            $table->timestamps();

            // Unique constraint for endpoint and db_name combination
            $table->unique(['endpoint', 'db_name']);
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('client_systems');
    }
};
