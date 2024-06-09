package org.saltedfish.concurrency.deamon;

public class DaemonDemo {

    public static void main(String[] args) throws InterruptedException {
        Thread t = new Thread(() -> {
            System.out.printf(String.valueOf(Thread.currentThread().isDaemon()));
        });
        t.setDaemon(true);
        t.start();

        Thread.sleep(3000);
    }
}
