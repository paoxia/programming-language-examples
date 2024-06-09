package org.saltedfish.concurrency.locksupport;

import java.util.concurrent.locks.LockSupport;

public class LockSupportDemo {

    public static void main(String[] args) throws InterruptedException {
        Thread t1 = new Thread(() -> {
            System.out.println(Thread.currentThread().getName() + "-" + "start");
            LockSupport.park();
            System.out.println(Thread.currentThread().getName() + "-" + "wake");
        });
        t1.start();

        Thread.sleep(1000);
        new Thread(() -> {
            LockSupport.unpark(t1);
        }).start();


    }
}
