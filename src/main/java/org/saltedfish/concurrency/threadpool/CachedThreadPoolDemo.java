package org.saltedfish.concurrency.threadpool;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

/**
 * 提交任务进线程池。
 * 因为corePoolSize为0的关系，不创建核心线程，线程池最大为Integer.MAX_VALUE。
 * 尝试将任务添加到SynchronousQueue队列。
 * 如果SynchronousQueue入列成功，等待被当前运行的线程空闲后拉取执行。如果当前没有空闲线程，那么就创建一个非核心线程，然后从SynchronousQueue拉取任务并在当前线程执行。
 * 如果SynchronousQueue已有任务在等待，入列操作将会阻塞。
 */

public class CachedThreadPoolDemo {

    public static void main(String[] args) {
        ExecutorService executor = Executors.newCachedThreadPool();

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
