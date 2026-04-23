#!/bin/bash
git pull

docker exec -it mysite bash -c "source ~/.cargo/env && cd /app && dx serve --release --platform web --addr 0.0.0.0"