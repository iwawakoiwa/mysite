#!/bin/bash
cd /home/iwa/web/mysite
git pull

docker exec mysite bash -c "pkill -9 dx || true"
sleep 1

docker exec -d mysite bash -c "source ~/.cargo/env && cd /app && dx serve --platform web --addr 0.0.0.0 --port 8080"
echo "✅ デプロイ完了"