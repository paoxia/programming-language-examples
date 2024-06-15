package org.saltedfish.concurrency.cas;

import java.util.concurrent.atomic.AtomicInteger;

public class CasDemo {

    public static AtomicInteger atomicCounter = new AtomicInteger(0);

    public static class IncrTask implements Runnable {

        @Override
        public void run() {
            for (int i = 0; i < 1000; i++) {
                int expectedValve;
                int newValue;
                do {
                    expectedValve = atomicCounter.get();
                    newValue = expectedValve + 1;
                } while (!atomicCounter.compareAndSet(expectedValve, newValue));
            }
        }
    }

    public static void main(String[] args) {

        for (int i = 0; i < 10; i++) {
            new Thread(new IncrTask()).start();
        }

        try {
            Thread.sleep(3000);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        System.out.println(atomicCounter.get());
    }
}
