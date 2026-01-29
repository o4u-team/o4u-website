<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

class AppUserDevice extends Model
{
    use HasFactory;

    protected $fillable = [
        'app_id',
        'client_system_id',
        'user_device_id',
        'username',
        'last_connected_at',
        'app_version',
    ];

    protected $casts = [
        'last_connected_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * Get the app.
     */
    public function app(): BelongsTo
    {
        return $this->belongsTo(App::class);
    }

    /**
     * Get the client system.
     */
    public function clientSystem(): BelongsTo
    {
        return $this->belongsTo(ClientSystem::class);
    }

    /**
     * Get the user device (device_id, device_info).
     */
    public function userDevice(): BelongsTo
    {
        return $this->belongsTo(UserDevice::class);
    }

    /**
     * Get the connection logs.
     */
    public function connectionLogs(): HasMany
    {
        return $this->hasMany(AppConnectionLog::class, 'app_user_device_id');
    }
}
