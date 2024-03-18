package org.saltedfish.concurrency.threadpool;


import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class ScheduledThreadPoolDemo {
    public static void main(String[] args) {
        // 创建一个定时执行任务的线程池
        ScheduledExecutorService executor = Executors.newScheduledThreadPool(1);

        // 定时执行任务
        executor.scheduleAtFixedRate(() -> {
            System.out.println("Executing task at " + System.currentTimeMillis());
        }, 0, 1, TimeUnit.SECONDS); // 初始延迟为 0，每隔 1 秒执行一次任务

        // 等待一段时间后关闭线程池
        try {
            Thread.sleep(5000);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        executor.shutdown();
    }
}
