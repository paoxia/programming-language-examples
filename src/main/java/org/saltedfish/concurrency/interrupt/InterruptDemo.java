package org.saltedfish.concurrency.interrupt;

import java.util.concurrent.atomic.AtomicBoolean;

public class InterruptDemo {
    static volatile boolean isStop = false;
    static AtomicBoolean atomicBoolean = new AtomicBoolean(false);

    public static void FuncVolatile() throws InterruptedException {
        new Thread(() -> {
            while (true) {
                if (isStop) {
                    System.out.println(Thread.currentThread().getName() + "-" + "isStop被修改为true");
                    break;
                }
                System.out.println(Thread.currentThread().getName() + "-" + "hello");
            }
        }, "t1").start();

        Thread.sleep(1000);

        new Thread(() -> {
            isStop = true;
        }, "t2").start();
    }

    public static void FuncAtomicBoolean() throws InterruptedException {
        new Thread(() -> {
            while (true) {
                if (atomicBoolean.get()) {
                    System.out.println(Thread.currentThread().getName() + "-" + "atomicBoolean被修改为true");
                    break;
                }
                System.out.println(Thread.currentThread().getName() + "-" + "hello");
            }
        }, "t1").start();

        Thread.sleep(1000);

        new Thread(() -> {
            atomicBoolean.set(true);
        }, "t2").start();
    }

    public static void FuncInterrupt() throws InterruptedException {
        Thread t1 = new Thread(() -> {
            while (true) {
                if (Thread.currentThread().isInterrupted()) {
                    System.out.println(Thread.currentThread().getName() + "-" + "该线程被请求中断");
                    break;
                }
                System.out.println(Thread.currentThread().getName() + "-" + "hello");
            }
        }, "t1");
        t1.start();


        Thread.sleep(1000);

        Thread t2 = new Thread(() -> {
            t1.interrupt();
        }, "t2");
        t2.start();
    }

    public static void FuncInterruptWithSleep() throws InterruptedException {
        Thread t1 = new Thread(() -> {
            while (true) {
                if (Thread.currentThread().isInterrupted()) {
                    System.out.println(Thread.currentThread().getName() + "-" + "该线程被请求中断");
                    break;
                }
                System.out.println(Thread.currentThread().getName() + "-" + "hello");
                try {
                    Thread.sleep(200);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    e.printStackTrace();
                }
            }
        }, "t1");
        t1.start();


        Thread.sleep(1000);

        Thread t2 = new Thread(() -> {
            t1.interrupt();
        }, "t2");
        t2.start();
    }

    public static void main(String[] args) throws InterruptedException {
        //FuncAtomicBoolean();
        //FuncInterrupt();
        FuncInterruptWithSleep();
    }
}
