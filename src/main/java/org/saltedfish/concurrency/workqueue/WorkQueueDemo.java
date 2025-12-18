package org.saltedfish.concurrency.workqueue;

import java.util.Arrays;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;
import java.util.stream.Collectors;

/**
 * 工作队列demo
 */
public class WorkQueueDemo {


    public static void main(String[] args) {
        List<Integer> ids = Arrays.asList(1, 2, 3, 4, 5);
        List<CompletableFuture<Load>> futures = ids.stream()
                .map(id -> CompletableFuture.supplyAsync(() -> Worker.work(id), Executors.newSingleThreadExecutor()))
                .collect(Collectors.toList());

        // 组合成一个“全部完成”的 Future
        CompletableFuture<Void> allDone = CompletableFuture.allOf(
                futures.toArray(new CompletableFuture[0])
        );


        // 阻塞等待所有完成
        try {
            allDone.get(10, TimeUnit.SECONDS); // 超时 10 秒
            // 收集结果（此时所有 future 已完成，join() 不会阻塞）
            List<Load> results = futures.stream()
                    .map(CompletableFuture::join) // 安全：已知已完成
                    .collect(Collectors.toList());

            System.out.println("Results: " + results);
        } catch (TimeoutException e) {
            // 取消所有未完成任务（可选）
            futures.forEach(f -> f.cancel(true));
        } catch (Exception e) {
            return;
        }
    }
}
