# VM options

```
--add-opens=java.base/java.util=ALL-UNNAMED
--add-opens=java.base/java.lang=ALL-UNNAMED
--add-opens=java.base/java.util.concurrent=ALL-UNNAMED
```

# check es

```bash
curl -X GET "http://localhost:9200/demo-flink/_search?pretty"
```