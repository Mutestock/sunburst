FROM node:19.7

COPY backend/js_rest /backend/js_rest
COPY config.toml /config.toml
COPY proto /proto
WORKDIR /backend/js_rest

RUN npm install -g pnpm

RUN pnpm install

EXPOSE 22553

CMD [ "pnpm", "start"]