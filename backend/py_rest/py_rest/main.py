from routes.router import app
from routes.article_routes import *
from routes.basic_routes import *
from utils.config import PY_REST_CONF

import uvicorn
import asyncio


async def main():
    config = uvicorn.Config("main:app", port=PY_REST_CONF["port"], log_level="info")
    server = uvicorn.Server(config)
    await server.serve()

if __name__ == "__main__":
    asyncio.run(main())