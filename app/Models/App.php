<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasManyThrough;
use Illuminate\Support\Str;

class App extends Model
{
    use HasFactory;

    protected $fillable = [
        'name',
        'uuid',
        'android_min_version',
        'android_current_version',
        'ios_min_version',
        'ios_current_version',
        'android_store_url',
        'apple_store_url',
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

        static::creating(function ($app) {
            if (empty($app->uuid)) {
                $app->uuid = (string) Str::uuid();
            }
        });
    }

    /**
     * Get the client systems associated with the app.
     */
    public function clientSystems(): BelongsToMany
    {
        return $this->belongsToMany(ClientSystem::class, 'app_client_system')
            ->withTimestamps();
    }

    /**
     * Get the app user devices (app + client_system + device links).
     */
    public function appUserDevices(): HasMany
    {
        return $this->hasMany(AppUserDevice::class, 'app_id');
    }

    /**
     * Get the connection logs for this app (via app_user_devices).
     */
    public function connectionLogs(): HasManyThrough
    {
        return $this->hasManyThrough(
            AppConnectionLog::class,
            AppUserDevice::class,
            'app_id',
            'app_user_device_id'
        );
    }

    /**
     * Compare two semantic versions
     * Returns: -1 if v1 < v2, 0 if equal, 1 if v1 > v2
     */
    public static function compareVersions(?string $v1, ?string $v2): int
    {
        if (empty($v1) || empty($v2)) {
            return 0;
        }

        return version_compare($v1, $v2);
    }

    /**
     * Validate semantic version format
     */
    public static function isValidSemanticVersion(?string $version): bool
    {
        if (empty($version)) {
            return true; // nullable
        }

        // Semantic version pattern: major.minor.patch (e.g., 1.0.0, 2.1.3)
        return preg_match('/^\d+\.\d+\.\d+$/', $version) === 1;
    }
}
