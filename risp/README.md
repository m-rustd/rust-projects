# 终端解析器
## 1、+ -
example: -
```shell
(+ 1 2 3)
```

```shell
(+ 1 2 (+ 3))
```
example: -
```shell
(- 1 2 3)
```

```shell
(- 1 2 (- 3))
```

## 2、> >= < <=
```shell
(> 1 2)
```

```shell
(>= 1 2)
```

```shell
(< 1 2)
```

```shell
(<= 1 2)
```

## 3、if def fn
```shell
(if true 2 3)
```

```shell
(def a 1)

(+ a 1)
```

```shell
(+ 100 ((fn (a) (+ a 100)) 100))
```