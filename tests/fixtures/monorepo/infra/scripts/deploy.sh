#!/usr/bin/env bash
set -euo pipefail

# deploy all services in order
SERVICES=(backend frontend)

for svc in "${SERVICES[@]}"; do
	echo "deploying $svc..."
	# placeholder: call real deploy here
done

echo "done"
