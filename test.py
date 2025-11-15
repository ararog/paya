import asyncio
import json
import vai
from vai import send


async def main():
    req = vai.Vai("https://jsonplaceholder.typicode.com").get("/todos/1")
    res = await send(req)
    data = json.loads(res.body())
    print(data)

    req = (
        req.post("/posts")
        .header("Content-Type", "application/json")
        .body(
            json.dumps(
                {"title": "Some title", "body": "Some body", "userId": 1}
            ).encode("utf-8")
        )
    )
    res = await send(req)
    data = json.loads(res.body())
    print(data)

    req = req.delete("/posts/1")
    res = await send(req)
    data = json.loads(res.body())
    print(data)

asyncio.run(main())
