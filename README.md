# paya

A minimum rest client based on deboa http client for rust

## Installation

```bash
pip install paya
```

## Usage

```python
import asyncio
import json
import paya
from paya import send


async def main():
    req = paya.Paya("https://jsonplaceholder.typicode.com").get("/todos/1")
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
```

## License

MIT


## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
