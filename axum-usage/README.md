# Axum Project
## 1、启动服务
```bash
cargo run
```

## 2、api
### 2.1、登录
```shell
curl  -X POST \
  'http://localhost:8000/login' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "email":"mz@qq.com",
  "password": "123456"
}'
```

### 2.2、获取todos
```shell
curl  -X GET \
  'http://localhost:3000/todos' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IjExMUBxcS5jb20iLCJleHAiOjE3MDE0MTI3NDJ9.5tlzJmB_3c4w687hT0J6Oc7y-8iCeSY16UXdaGxz78w' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "title": "title2"
}'
```

### 2.2、创建todo
```shell
curl  -X POST \
  'http://localhost:3000/todos' \
  --header 'Accept: */*' \
  --header 'User-Agent: Thunder Client (https://www.thunderclient.com)' \
  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IjExMUBxcS5jb20iLCJleHAiOjE3MDE0MTI3NDJ9.5tlzJmB_3c4w687hT0J6Oc7y-8iCeSY16UXdaGxz78w' \
  --header 'Content-Type: application/json' \
  --data-raw '{
  "title": "title2"
}'
```
