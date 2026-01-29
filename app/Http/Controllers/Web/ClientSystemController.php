<?php

namespace App\Http\Controllers\Web;

use App\Models\App;
use App\Models\Client;
use App\Models\ClientSystem;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\Validation\Rule;
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
                Rule::unique('client_systems')->where(function ($query) use ($request) {
                    return $query->where('db_name', $request->db_name);
                }),
            ],
            'db_name' => 'required|string|max:255',
            'status' => 'required|in:active,inactive,expired',
        ], [
            'endpoint.unique' => 'Cặp endpoint và db_name đã tồn tại.',
            'endpoint.url' => 'Endpoint phải là một URL hợp lệ (http hoặc https).',
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
                Rule::unique('client_systems')->where(function ($query) use ($request) {
                    return $query->where('db_name', $request->db_name);
                })->ignore($clientSystem->id),
            ],
            'db_name' => 'required|string|max:255',
            'status' => 'required|in:active,inactive,expired',
        ], [
            'endpoint.unique' => 'Cặp endpoint và db_name đã tồn tại.',
            'endpoint.url' => 'Endpoint phải là một URL hợp lệ (http hoặc https).',
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
    public function getApps(ClientSystem $clientSystem): JsonResponse
    {
        $apps = App::where('status', 'active')
            ->orderBy('name')
            ->get(['id', 'name', 'uuid']);

        $assignedAppIds = $clientSystem->apps()->pluck('apps.id')->toArray();

        $appsWithStatus = $apps->map(function ($app) use ($assignedAppIds) {
            return [
                'id' => $app->id,
                'name' => $app->name,
                'uuid' => $app->uuid,
                'assigned' => in_array($app->id, $assignedAppIds),
            ];
        });

        return response()->json([
            'apps' => $appsWithStatus,
            'clientSystem' => [
                'id' => $clientSystem->id,
                'name' => $clientSystem->name,
            ],
        ]);
    }

    /**
     * Sync apps for a client system.
     */
    public function syncApps(Request $request, ClientSystem $clientSystem): RedirectResponse
    {
        $validated = $request->validate([
            'app_ids' => 'array',
            'app_ids.*' => 'exists:apps,id',
        ]);

        $appIds = $validated['app_ids'] ?? [];

        // Sync apps (this will add new and remove old associations)
        $clientSystem->apps()->sync($appIds);

        return redirect()->back()
            ->with('success', 'Cập nhật phân quyền app thành công.');
    }
}
