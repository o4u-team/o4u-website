<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Support\Str;

class ClientSystem extends Model
{
    use HasFactory;

    protected $fillable = [
        'name',
        'uuid',
        'client_id',
        'endpoint',
        'db_name',
        'status',
    ];

    protected $casts = [
        'created_at' => 'datetime',
        'updated_at' => 'datetime',
    ];

    /**
     * Boot method to auto-generate UUID
     */
    protected static function boot()
    {
        parent::boot();

        static::creating(function ($clientSystem) {
            if (empty($clientSystem->uuid)) {
                $clientSystem->uuid = (string) Str::uuid();
            }
        });
    }

    /**
     * Get the client that owns the system.
     */
    public function client(): BelongsTo
    {
        return $this->belongsTo(Client::class);
    }

    /**
     * Get the apps associated with the client system.
     */
    public function apps(): BelongsToMany
    {
        return $this->belongsToMany(App::class, 'app_client_system')
            ->withTimestamps();
    }

    /**
     * Get the app user devices (app + client_system + device links).
     */
    public function appUserDevices(): HasMany
    {
        return $this->hasMany(AppUserDevice::class);
    }

    /**
     * Get the connection logs for this client system (via app_user_devices).
     */
    public function connectionLogs(): HasManyThrough
    {
        return $this->hasManyThrough(
            AppConnectionLog::class,
            AppUserDevice::class,
            'client_system_id',
            'app_user_device_id'
        );
    }
}
