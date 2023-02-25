FROM deno:latest

COPY backend/ts_rest/deno_rest /backend/ts_rest/deno_rest
COPY config /config.toml
COPY proto /proto

CMD ["deno", "run", "--allow-read", "--allow-net", "src/main.ts" ]