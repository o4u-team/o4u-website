<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class AppConnectionLog extends Model
{
    use HasFactory;

    protected $fillable = [
        'app_user_device_id',
        'app_version',
        'ip_address',
    ];

    protected $casts = [
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * Get the app user device (app, client_system, user_device, username from here).
     */
    public function appUserDevice(): BelongsTo
    {
        return $this->belongsTo(AppUserDevice::class);
    }
}
