from routes.router import app


@app.get("/health")
async def health_check_route():
    return "ok"

