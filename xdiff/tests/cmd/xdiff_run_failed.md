# xdiff run failed

```trycmd
$ xdiff run -p todo -c  ./fixtures/bad.yml -e a=10 -e @b=2 -e %c=3 -e m=10
failed to validate profile: todo

Caused by:
    0: req1 failed to validate
    1: Params must be an object but got
       fgasfsdf
       

```
