package org.saltedfish.concurrency.semaphore;


import java.util.concurrent.Semaphore;

public class SemaphoreDemo {

    private static Semaphore semaphore = new Semaphore(1);
    private static int i = 1;

    class SemaTask implements Runnable {


        @Override
        public void run() {
            try {
                semaphore.acquire();
                System.out.println(i++);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            semaphore.release();
        }
    }

    public static void main(String[] args) {
        SemaTask task = new SemaphoreDemo().new SemaTask();
        Thread thread1 = new Thread(task);
        Thread thread2 = new Thread(task);
        thread1.start();
        thread2.start();
    }
}
