#!/bin/bash
function log() {
    echo "$(date +%s) $*"
}
if [[ ! -z "${DEBUG}" ]]
then
    echo "DEBUG requested, sleeping infinity."
    /bin/sleep infinity
fi

if [[ ! -d "/app/templates/dbrs-error-no-code.html.hbs" ]];
then
    log "a default template was not detected in templates folder, reseeding defaults into container filesystem."
    mkdir -p /app/templates /app/public
    cp -rv /app/default_templates/*.hbs /app/templates
    cp -rv /app/default_public/* /app/public
fi

export ROCKET_ENV=${app_env:-prod}
cd /app
./default-backend-rs
