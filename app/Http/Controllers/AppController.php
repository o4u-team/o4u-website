<?php

namespace App\Http\Controllers;

use App\Models\App;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Inertia\Inertia;
use Inertia\Response;

class AppController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index(Request $request): Response
    {
        $query = App::query();

        // Search
        if ($request->has('search') && $request->search) {
            $query->where('name', 'like', '%' . $request->search . '%')
                  ->orWhere('uuid', 'like', '%' . $request->search . '%');
        }

        // Filter by status
        if ($request->has('status') && $request->status) {
            $query->where('status', $request->status);
        }

        // Sort
        $sortField = $request->get('sort_field', 'created_at');
        $sortDirection = $request->get('sort_direction', 'desc');
        $query->orderBy($sortField, $sortDirection);

        $apps = $query->paginate(10)->withQueryString();

        return Inertia::render('Apps/Index', [
            'apps' => $apps,
            'filters' => [
                'search' => $request->search,
                'status' => $request->status,
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
        return Inertia::render('Apps/Form', [
            'app' => null,
        ]);
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request): RedirectResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'android_min_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'android_current_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'ios_min_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'ios_current_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'android_store_url' => 'nullable|url|max:500',
            'apple_store_url' => 'nullable|url|max:500',
            'status' => 'required|in:active,maintenance,inactive',
        ], [
            'android_min_version.regex' => 'Android min version must be in semantic version format (e.g., 1.0.0)',
            'android_current_version.regex' => 'Android current version must be in semantic version format (e.g., 1.0.0)',
            'ios_min_version.regex' => 'iOS min version must be in semantic version format (e.g., 1.0.0)',
            'ios_current_version.regex' => 'iOS current version must be in semantic version format (e.g., 1.0.0)',
            'android_store_url.url' => 'Android store URL must be a valid URL',
            'apple_store_url.url' => 'Apple store URL must be a valid URL',
        ]);

        // Validate Android: min_version <= current_version
        if (!empty($validated['android_min_version']) && !empty($validated['android_current_version'])) {
            if (App::compareVersions($validated['android_min_version'], $validated['android_current_version']) > 0) {
                return back()->withErrors([
                    'android_min_version' => 'Android min version must be less than or equal to current version',
                ])->withInput();
            }
        }

        // Validate iOS: min_version <= current_version
        if (!empty($validated['ios_min_version']) && !empty($validated['ios_current_version'])) {
            if (App::compareVersions($validated['ios_min_version'], $validated['ios_current_version']) > 0) {
                return back()->withErrors([
                    'ios_min_version' => 'iOS min version must be less than or equal to current version',
                ])->withInput();
            }
        }

        App::create($validated);

        return redirect()->route('apps.index')
            ->with('success', 'App created successfully.');
    }

    /**
     * Show the form for editing the specified resource.
     */
    public function edit(App $app): Response
    {
        return Inertia::render('Apps/Form', [
            'app' => $app,
        ]);
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, App $app): RedirectResponse
    {
        $validated = $request->validate([
            'name' => 'required|string|max:255',
            'android_min_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'android_current_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'ios_min_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'ios_current_version' => 'nullable|string|regex:/^\d+\.\d+\.\d+$/',
            'android_store_url' => 'nullable|url|max:500',
            'apple_store_url' => 'nullable|url|max:500',
            'status' => 'required|in:active,maintenance,inactive',
        ], [
            'android_min_version.regex' => 'Android min version must be in semantic version format (e.g., 1.0.0)',
            'android_current_version.regex' => 'Android current version must be in semantic version format (e.g., 1.0.0)',
            'ios_min_version.regex' => 'iOS min version must be in semantic version format (e.g., 1.0.0)',
            'ios_current_version.regex' => 'iOS current version must be in semantic version format (e.g., 1.0.0)',
            'android_store_url.url' => 'Android store URL must be a valid URL',
            'apple_store_url.url' => 'Apple store URL must be a valid URL',
        ]);

        // Validate Android: min_version <= current_version
        if (!empty($validated['android_min_version']) && !empty($validated['android_current_version'])) {
            if (App::compareVersions($validated['android_min_version'], $validated['android_current_version']) > 0) {
                return back()->withErrors([
                    'android_min_version' => 'Android min version must be less than or equal to current version',
                ])->withInput();
            }
        }

        // Validate iOS: min_version <= current_version
        if (!empty($validated['ios_min_version']) && !empty($validated['ios_current_version'])) {
            if (App::compareVersions($validated['ios_min_version'], $validated['ios_current_version']) > 0) {
                return back()->withErrors([
                    'ios_min_version' => 'iOS min version must be less than or equal to current version',
                ])->withInput();
            }
        }

        $app->update($validated);

        return redirect()->route('apps.index')
            ->with('success', 'App updated successfully.');
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(App $app): RedirectResponse
    {
        $app->delete();

        return redirect()->route('apps.index')
            ->with('success', 'App deleted successfully.');
    }
}
