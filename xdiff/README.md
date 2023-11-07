## xdiff
### run
> 对比两个请求响应的不同
```shell
xdiff run -p todo -c  ./fixtures/bad.yml -e a=10 -e @b=2 -e %c=3 
```
### parse
> 输入URL转换为yml格式
```shell
xdiff parse
```
## xreq
> 打印请求的响应
### run
```shell
xreq run -p todo-new -c fixtures/req.yml -e a=100
```

### parse
> 输入URL转换为yml格式
```shell
xreq parse
```

## 测试
### test
```shell
cargo test
```
### trycmd
```shell
cargo test --test trycmd_test
```