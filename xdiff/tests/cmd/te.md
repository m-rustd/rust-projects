1   1    | HTTP/2.0 200 OK
2   2    | content-type: "application/json; charset=utf-8"
3        |-content-length: "83"
    3    |+content-length: "99"
4   4    | x-powered-by: "Express"
5   5    | x-ratelimit-limit: "1000"
6   6    | vary: "Origin, Accept-Encoding"
--------------------------------------------------------------------------------
16  16   | 
17  17   | {
18  18   |   "completed": false,
19       |-  "title": "delectus aut autem",
    19   |+  "title": "quis ut nam facilis et officia qui",
20  20   |   "userId": 1
21  21   | }
