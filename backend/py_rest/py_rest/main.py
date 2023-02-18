from routes.router import app
from routes.article_routes import *
from routes.basic_routes import *

import uvicorn
import asyncio


async def main():
    config = uvicorn.Config("main:app", port=22551, log_level="info")
    server = uvicorn.Server(config)
    await server.serve()

if __name__ == "__main__":
    asyncio.run(main())