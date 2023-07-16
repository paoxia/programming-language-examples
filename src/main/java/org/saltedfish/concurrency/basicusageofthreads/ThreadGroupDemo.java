package org.saltedfish.concurrency.basicusageofthreads;

/**
 *
 * 线程组
 */
public class ThreadGroupDemo {
    public static void main(String[] args) {
        Thread newThread = new Thread(() -> System.out.println("Thread Group Name:"
                + Thread.currentThread().getThreadGroup().getName()));
        newThread.start();
        System.out.println("Main Thread Name:" + Thread.currentThread().getName());
    }
}
