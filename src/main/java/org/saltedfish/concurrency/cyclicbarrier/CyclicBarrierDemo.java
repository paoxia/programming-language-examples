package org.saltedfish.concurrency.cyclicbarrier;

import java.util.concurrent.BrokenBarrierException;
import java.util.concurrent.CyclicBarrier;

public class CyclicBarrierDemo {

    public static void main(String[] args) {
        CyclicBarrier cyclicBarrier = new CyclicBarrier(3, () -> System.out.println("all work reach barrier point"));

        Thread thread1 = new Thread(new Worker(cyclicBarrier, "work1"));
        Thread thread2 = new Thread(new Worker(cyclicBarrier, "work2"));
        Thread thread3 = new Thread(new Worker(cyclicBarrier, "work3"));

        thread1.start();
        thread2.start();
        thread3.start();

    }
}

class Worker implements Runnable {

    private final CyclicBarrier cyclicBarrier;
    private final String name;

    public Worker(CyclicBarrier cyclicBarrier, String name) {
        this.cyclicBarrier = cyclicBarrier;
        this.name = name;
    }


    @Override
    public void run() {
        try {
            System.out.println(this.name + " reach barrier point");
            cyclicBarrier.await();

            System.out.println("continue");
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        } catch (BrokenBarrierException e) {
            throw new RuntimeException(e);
        }
    }
}
