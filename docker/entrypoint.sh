#!/bin/bash
function log() {
    echo "$(date +%s) $*"
}
if [[ ! -z "${DEBUG}" ]]
then
    log "DEBUG requested, sleeping infinity."
    /bin/sleep infinity
fi

log "updating any files that don't already exist in the volume store"
mkdir -p /app/templates /app/public
cp -nv /app/default_templates/*.hbs /app/templates
cp -rnv /app/default_public/* /app/public

if [[ "${app_env}" != "dev" ]]
then
    export ROCKET_ENV=${ROCKET_ENV:-prod}
    export RUST_LOG=${RUST_LOG:=warn}
else
    export ROCKET_ENV=${ROCKET_ENV:-dev}
    export RUST_LOG=${RUST_LOG:=info}
fi

cd /app
./default-backend-rs
