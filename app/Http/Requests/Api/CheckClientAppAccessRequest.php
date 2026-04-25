<?php

declare(strict_types=1);

namespace App\Http\Requests\Api;

use Illuminate\Contracts\Validation\ValidationRule;
use Illuminate\Foundation\Http\FormRequest;

class CheckClientAppAccessRequest extends FormRequest
{
    public function authorize(): bool
    {
        return true;
    }

    /**
     * @return array<string, ValidationRule|array<int, ValidationRule|string>|string>
     */
    public function rules(): array
    {
        $app = $this->get('app');
        $isPublicApp = is_object($app) && property_exists($app, 'allow_public') && (bool) $app->allow_public;

        return [
            'client_domain' => [$isPublicApp ? 'nullable' : 'required', 'string'],
            'db_name' => [$isPublicApp ? 'nullable' : 'required', 'string'],
        ];
    }
}

