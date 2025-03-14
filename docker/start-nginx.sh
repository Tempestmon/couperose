#!/bin/sh

envsubst '${API_TOKEN}' </etc/nginx/templates/messenger.conf >/etc/nginx/sites-enabled/messenger.conf

exec nginx -g "daemon off;"
