<?php

namespace App\Http\Controllers\Web;

use App\Models\App;
use App\Models\AppConnectionLog;
use App\Models\AppUserDevice;
use App\Models\ClientSystem;
use Illuminate\Http\Request;
use Inertia\Inertia;
use Inertia\Response;
use App\Http\Controllers\Controller;

class AppUsageController extends Controller
{
    /**
     * Display app usage: app_user_devices and connection logs.
     */
    public function index(Request $request): Response
    {
        // Tab: app_user_devices
        $appUserDevicesQuery = AppUserDevice::with(['app', 'clientSystem', 'userDevice']);

        if ($request->filled('app_id')) {
            $appUserDevicesQuery->where('app_id', $request->app_id);
        }
        if ($request->filled('client_system_id')) {
            $appUserDevicesQuery->where('client_system_id', $request->client_system_id);
        }
        if ($request->filled('search_devices')) {
            $term = '%' . $request->search_devices . '%';
            $appUserDevicesQuery->where(function ($q) use ($term) {
                $q->where('username', 'like', $term)
                    ->orWhereHas('userDevice', fn ($q) => $q->where('device_id', 'like', $term));
            });
        }

        $appUserDevices = $appUserDevicesQuery->orderByDesc('last_connected_at')->paginate(10, ['*'], 'devices_page')->withQueryString();

        // Tab: app_connection_logs
        $logsQuery = AppConnectionLog::with(['appUserDevice.app', 'appUserDevice.clientSystem', 'appUserDevice.userDevice']);

        if ($request->filled('log_app_id')) {
            $logsQuery->whereHas('appUserDevice', fn ($q) => $q->where('app_id', $request->log_app_id));
        }
        if ($request->filled('log_client_system_id')) {
            $logsQuery->whereHas('appUserDevice', fn ($q) => $q->where('client_system_id', $request->log_client_system_id));
        }

        $connectionLogs = $logsQuery->orderByDesc('created_at')->paginate(15, ['*'], 'logs_page')->withQueryString();

        // Filter options
        $apps = App::where('status', 'active')->orderBy('name')->get(['id', 'name']);
        $clientSystems = ClientSystem::with('client:id,name')->where('status', 'active')->orderBy('name')->get(['id', 'name', 'client_id']);

        return Inertia::render('AppUsage/Index', [
            'appUserDevices' => $appUserDevices,
            'connectionLogs' => $connectionLogs,
            'apps' => $apps,
            'clientSystems' => $clientSystems,
            'filters' => [
                'app_id' => $request->app_id,
                'client_system_id' => $request->client_system_id,
                'search_devices' => $request->search_devices,
                'log_app_id' => $request->log_app_id,
                'log_client_system_id' => $request->log_client_system_id,
            ],
        ]);
    }
}
