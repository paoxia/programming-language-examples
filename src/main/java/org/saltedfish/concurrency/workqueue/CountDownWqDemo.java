package org.saltedfish.concurrency.workqueue;


import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

import lombok.extern.slf4j.Slf4j;

@Slf4j
public class CountDownWqDemo {

    public static void main(String[] args) {

        List<Integer> ids = Arrays.asList(1, 2, 3, 4, 5);

        CountDownLatch countDownLatch = new CountDownLatch(5);

        ConcurrentHashMap<String, Integer> concurrentHashMap = new ConcurrentHashMap<>();


        for (Integer num : ids) {
            new Thread(() -> {
                try {
                    concurrentHashMap.put(num.toString(), num);
                } catch (Exception e) {

                } finally {
                    countDownLatch.countDown();
                }
            }).start();
        }


        try {
            countDownLatch.await(10, TimeUnit.SECONDS);
        } catch (Exception e) {
            log.error("CountDownWqDemo", e);
        }
        System.out.println(concurrentHashMap.values());
    }
}
