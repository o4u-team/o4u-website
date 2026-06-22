#!/bin/sh
set -e

cd /var/www/html

mkdir -p \
    storage/app/public \
    storage/app/private \
    storage/framework/cache/data \
    storage/framework/sessions \
    storage/framework/views \
    storage/logs \
    bootstrap/cache

chown -R www-data:www-data storage bootstrap/cache
chmod -R ug+rwX storage bootstrap/cache

if [ -f artisan ]; then
    if [ "${RUN_MIGRATIONS:-false}" = "true" ]; then
        php artisan migrate --force --no-interaction
    fi

    if [ "${APP_ENV:-production}" = "production" ]; then
        php artisan config:cache --no-interaction
        php artisan route:cache --no-interaction
        php artisan view:cache --no-interaction
    fi
fi

exec "$@"
