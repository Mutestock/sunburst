FROM node:19.7

COPY backend/js_rest /backend/js_rest
COPY config.toml /config.toml
COPY proto /proto

RUN pnpm install

EXPOSE 22553

CMD [ "pnpm", "start"]