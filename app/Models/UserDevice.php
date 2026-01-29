<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;

class UserDevice extends Model
{
    use HasFactory;

    protected $fillable = [
        'device_id',
        'device_info',
        'last_connected_at',
    ];

    protected $casts = [
        'device_info' => 'array',
        'last_connected_at' => 'datetime',
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * Get the app_user_devices (links to app + client_system + username).
     */
    public function appUserDevices(): HasMany
    {
        return $this->hasMany(AppUserDevice::class);
    }
}
