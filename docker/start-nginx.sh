#!/bin/sh

mkdir -p /etc/nginx/sites-enabled

for template in /etc/nginx/templates/*.conf; do
  filename=$(basename "$template")
  envsubst '${API_TOKEN}' <"$template" >"/etc/nginx/sites-enabled/$filename"
done

exec nginx -g "daemon off;"
