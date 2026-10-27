<?php

namespace App\Http\Controllers\Web;

use App\Models\App;
use App\Models\Client;
use App\Models\ClientSystem;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\Validation\Rule;
use Illuminate\Validation\ValidationException;
use Inertia\Inertia;
use Inertia\Response;
use App\Http\Controllers\Controller;

class ClientSystemController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index(Request $request): Response
    {
        $query = ClientSystem::with('client');

        // Search
        if ($request->has('search') && $request->search) {
            $searchTerm = '%' . $request->search . '%';
            $query->where(function ($q) use ($searchTerm) {
                $q->where('name', 'like', $searchTerm)
                    ->orWhere('uuid', 'like', $searchTerm)
                    ->orWhere('endpoint', 'like', $searchTerm)
                    ->orWhere('db_name', 'like', $searchTerm);
            });
        }

        // Filter by status
        if ($request->has('status') && $request->status) {
            $query->where('status', $request->status);
        }

        // Filter by client
        if ($request->has('client_id') && $request->client_id) {
            $query->where('client_id', $request->client_id);
        }

        // Sort
        $sortField = $request->get('sort_field', 'created_at');
        $sortDirection = $request->get('sort_direction', 'desc');
        $query->orderBy($sortField, $sortDirection);

        $clientSystems = $query->paginate(10)->withQueryString();

        // Get all clients for filter dropdown
        $clients = Client::orderBy('name')->get(['id', 'name']);

        return Inertia::render('ClientSystems/Index', [
            'clientSystems' => $clientSystems,
            'clients' => $clients,
            'filters' => [
                'search' => $request->search,
                'status' => $request->status,
                'client_id' => $request->client_id,
                'sort_field' => $sortField,
                'sort_direction' => $sortDirection,
            ],
        ]);
    }

    /**
     * Show the form for creating a new resource.
     */
    public function create(): Response
    {
        $clients = Client::where('status', 'active')->orderBy('name')->get(['id', 'name']);

        return Inertia::render('ClientSystems/Form', [
            'clientSystem' => null,
            'clients' => $clients,
        ]);
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request): RedirectResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'client_id' => 'required|exists:clients,id',
            'endpoint' => [
                'required',
                'string',
                'max:255',
                'url:http,https',
            ],
            'webapp_endpoint' => [
                'nullable',
                'string',
                'max:255',
                'url:http,https',
            ],
            'allow_get_info' => 'boolean',
            'db_name' => 'required|string|max:255',
            'status' => 'required|in:active,inactive,expired',
        ], [
            'endpoint.url' => 'Endpoint phải là một URL hợp lệ (http hoặc https).',
            'webapp_endpoint.url' => 'Webapp endpoint phải là một URL hợp lệ (http hoặc https).',
        ]);

        ClientSystem::create($validated);

        return redirect()->route('client-systems.index')
            ->with('success', 'Client System created successfully.');
    }

    /**
     * Show the form for editing the specified resource.
     */
    public function edit(ClientSystem $clientSystem): Response
    {
        $clients = Client::where('status', 'active')->orderBy('name')->get(['id', 'name']);

        return Inertia::render('ClientSystems/Form', [
            'clientSystem' => $clientSystem,
            'clients' => $clients,
        ]);
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, ClientSystem $clientSystem): RedirectResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'client_id' => 'required|exists:clients,id',
            'endpoint' => [
                'required',
                'string',
                'max:255',
                'url:http,https',
            ],
            'webapp_endpoint' => [
                'nullable',
                'string',
                'max:255',
                'url:http,https',
            ],
            'allow_get_info' => 'boolean',
            'db_name' => 'required|string|max:255',
            'status' => 'required|in:active,inactive,expired',
        ], [
            'endpoint.url' => 'Endpoint phải là một URL hợp lệ (http hoặc https).',
            'webapp_endpoint.url' => 'Webapp endpoint phải là một URL hợp lệ (http hoặc https).',
        ]);

        $clientSystem->update($validated);

        return redirect()->route('client-systems.index')
            ->with('success', 'Client System updated successfully.');
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(ClientSystem $clientSystem): RedirectResponse
    {
        $clientSystem->delete();

        return redirect()->route('client-systems.index')
            ->with('success', 'Client System deleted successfully.');
    }

    /**
     * Get apps with their assignment status for a client system.
     */
    public function appInfo(ClientSystem $clientSystem): Response
    {
        return Inertia::render('ClientSystems/AppInfo', [
            'clientSystem' => $clientSystem->only(['id', 'name', 'uuid', 'endpoint', 'db_name']),
            'apps' => $this->appsFor($clientSystem),
        ]);
    }

    public function getApps(ClientSystem $clientSystem): JsonResponse
    {
        return response()->json([
            'apps' => $this->appsFor($clientSystem),
            'clientSystem' => $clientSystem->only(['id', 'name']),
        ]);
    }

    private function appsFor(ClientSystem $clientSystem): \Illuminate\Support\Collection
    {
        $apps = App::where(function ($query) use ($clientSystem) {
                $query->where('status', 'active')
                    ->orWhereHas('clientSystems', fn ($systems) => $systems->where('client_systems.id', $clientSystem->id));
            })
            ->orderBy('name')
            ->get(array_merge(['id', 'name', 'uuid'], App::CLIENT_SETTINGS));

        $assignedApps = $clientSystem->apps()->get()->keyBy('id');

        return $apps->map(function ($app) use ($assignedApps) {
            $settings = [];
            foreach (App::CLIENT_SETTINGS as $field) {
                $settings[$field] = $assignedApps->get($app->id)?->pivot?->getAttribute($field);
                $settings['default_'.$field] = $app->getAttribute($field);
            }

            return [
                ...$settings,
                'id' => $app->id,
                'name' => $app->name,
                'uuid' => $app->uuid,
                'assigned' => $assignedApps->has($app->id),
            ];
        });

    }

    /**
     * Sync apps for a client system.
     */
    public function syncApps(Request $request, ClientSystem $clientSystem): RedirectResponse
    {
        $validated = $request->validate([
            'app_ids' => 'array',
            'app_ids.*' => 'integer|distinct|exists:apps,id',
            'app_settings' => 'array',
            'app_settings.*' => 'array:'.implode(',', App::CLIENT_SETTINGS),
            'app_settings.*.webapp_url' => 'nullable|string|max:255|url:http,https',
            'app_settings.*.android_min_version' => ['nullable', 'string', 'max:255', 'regex:/^\d+\.\d+\.\d+$/'],
            'app_settings.*.android_current_version' => ['nullable', 'string', 'max:255', 'regex:/^\d+\.\d+\.\d+$/'],
            'app_settings.*.ios_min_version' => ['nullable', 'string', 'max:255', 'regex:/^\d+\.\d+\.\d+$/'],
            'app_settings.*.ios_current_version' => ['nullable', 'string', 'max:255', 'regex:/^\d+\.\d+\.\d+$/'],
            'app_settings.*.android_store_url' => 'nullable|string|max:500|url:http,https',
            'app_settings.*.apple_store_url' => 'nullable|string|max:500|url:http,https',
            'webapp_urls' => 'array',
            'webapp_urls.*' => 'nullable|string|max:255|url:http,https',
        ]);

        $appIds = $validated['app_ids'] ?? [];

        // Sync apps (this will add new and remove old associations)
        // Omitted URLs preserve existing configuration for older callers.
        $assignments = [];
        foreach ($appIds as $appId) {
            $assignments[$appId] = array_key_exists($appId, $validated['webapp_urls'] ?? [])
                ? ['webapp_url' => $validated['webapp_urls'][$appId]]
                : [];
        }
        foreach ($appIds as $appId) {
            $assignments[$appId] = array_merge($assignments[$appId], $validated['app_settings'][$appId] ?? []);
        }
        $apps = App::whereIn('id', $appIds)->get();
        $existing = $clientSystem->apps()->get()->keyBy('id');
        foreach ($apps as $app) {
            foreach (['android', 'ios'] as $platform) {
                $versions = [];
                foreach (['min_version', 'current_version'] as $suffix) {
                    $field = $platform.'_'.$suffix;
                    $value = array_key_exists($field, $assignments[$app->id])
                        ? $assignments[$app->id][$field]
                        : $existing->get($app->id)?->pivot?->getAttribute($field);
                    $versions[$suffix] = is_string($value) && trim($value) !== '' ? $value : $app->getAttribute($field);
                }
                if ($versions['min_version'] && $versions['current_version']
                    && App::compareVersions($versions['min_version'], $versions['current_version']) > 0) {
                    throw ValidationException::withMessages([
                        "app_settings.{$app->id}.{$platform}_min_version" => 'Phiên bản tối thiểu phải nhỏ hơn hoặc bằng phiên bản hiện tại.',
                    ]);
                }
            }
        }
        $clientSystem->apps()->sync($assignments);

        return redirect()->back()
            ->with('success', 'Cập nhật phân quyền app thành công.');
    }
}
