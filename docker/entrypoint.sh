#!/bin/bash
if [[ ! -z "${DEBUG}" ]]
then
    echo "DEBUG requested, sleeping infinity."
    /bin/sleep infinity
fi
export ROCKET_ENV=${app_env:-prod}
cd /app
./default-backend-rs
