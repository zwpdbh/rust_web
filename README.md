# Chapter 03

## Testing CORS response

```shell
curl -X OPTIONS localhost:3030/questions \
  -H "Access-Control-Request-Method: PUT" \
  -H "Access-Control-request-Headers: content-type" \
  -H "Origin: https://not-origin.io" -verbose
```
