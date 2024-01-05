package org.saltedfish.concurrency.countdownlatch;



import java.util.concurrent.CountDownLatch;

public class CountDownLatchDemo {
    public static void main(String[] args) {
        CountDownLatch countDownLatch = new CountDownLatch(2);
        Thread thread1 = new Thread(new Worker(countDownLatch, "work1"));
        Thread thread2 = new Thread(new Worker(countDownLatch, "work2"));
        Thread thread3 = new Thread(new Worker(countDownLatch, "work3"));

        thread1.start();
        thread2.start();
        thread3.start();

        try {
            countDownLatch.await();
            System.out.println("CountDownLatch finish");
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    }
}


class Worker implements Runnable {

    private final CountDownLatch countDownLatch;
    private final String name;

    public Worker(CountDownLatch countDownLatch, String name) {
        this.countDownLatch = countDownLatch;
        this.name = name;
    }

    @Override
    public void run() {
        countDownLatch.countDown();
        System.out.println(this.name + " finish task");
    }
}
