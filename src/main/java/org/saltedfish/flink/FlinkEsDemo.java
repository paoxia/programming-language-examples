package org.saltedfish.flink;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.apache.flink.api.common.RuntimeExecutionMode;
import org.apache.flink.streaming.api.datastream.DataStream;
import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;
import org.apache.flink.streaming.connectors.elasticsearch7.ElasticsearchSink;
import org.apache.http.HttpHost;
import org.elasticsearch.action.index.IndexRequest;
import org.elasticsearch.client.Requests;

public class FlinkEsDemo {

    public static void main(String[] args) throws Exception {
        StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();

        // 建议显式设置，方便本地调试查看结果
        env.setRuntimeMode(RuntimeExecutionMode.STREAMING);
        env.setParallelism(1);

        // 1. 构造数据 (显式使用 HashMap 避免序列化问题)
        List<Map<String, Object>> data = new ArrayList<>();
        for (int i = 1; i <= 5; i++) {
            Map<String, Object> event = new HashMap<>();
            event.put("id", String.valueOf(i)); // 建议 id 转为 String
            event.put("msg", "Hello Flink ES #" + i);
            event.put("ts", System.currentTimeMillis());
            data.add(event);
        }

        DataStream<Map<String, Object>> stream = env.fromCollection(data);

        // 2. 配置 ES Sink
        List<HttpHost> httpHosts = new ArrayList<>();
        httpHosts.add(new HttpHost("127.0.0.1", 9200, "http"));

        ElasticsearchSink.Builder<Map<String, Object>> esSinkBuilder = new ElasticsearchSink.Builder<>(
                httpHosts,
                (element, ctx, indexer) -> {
                    // 构造 IndexRequest
                    IndexRequest request = Requests.indexRequest()
                            .index("demo-flink")
                            .id(element.get("id").toString()) // 手动指定 ID 可实现 Upsert
                            .source(element); // 也可以显式指定 XContentType.JSON

                    indexer.add(request);
                }
        );

        // 3. 性能优化配置 (非常重要)
        esSinkBuilder.setBulkFlushMaxActions(1); // 测试阶段设为 1，立即看到结果
        // esSinkBuilder.setBulkFlushInterval(1000L); // 生产环境建议 1-5s

        // 4. 执行
        stream.addSink(esSinkBuilder.build());

        System.out.println("Job starting...");
        env.execute("Flink-ES-Demo");
    }
}