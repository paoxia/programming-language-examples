package org.saltedfish.concurrency.threadpool;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;


/**
 * 有且仅有一个核心线程（ corePoolSize == maximumPoolSize=1），
 * 使用了LinkedBlockingQueue（容量很大），所以，不会创建非核心线程。
 * 所有任务按照先来先执行的顺序执行。
 * 如果这个唯一的线程不空闲，那么新来的任务会存储在任务队列里等待执行。
 */

public class SingleThreadExecutorDemo {

    public static void main(String[] args) {
        ExecutorService executor = Executors.newSingleThreadExecutor();

        // 提交任务给线程池执行
        for (int i = 0; i < 10; i++) {
            final int taskNumber = i;
            executor.execute(() -> {
                System.out.println("Task " + taskNumber + " is running on thread " + Thread.currentThread().getName());
                try {
                    Thread.sleep(1000); // 模拟任务执行
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            });
        }

        // 关闭线程池
        executor.shutdown();
    }
}
