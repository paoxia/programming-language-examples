package org.saltedfish.concurrency.basicusageofthreads;

/**
 *
 * 继承thread实现多线程
 */

class ExtendsThread extends Thread {
    @Override
    public void run() {
        System.out.println("New thread by ExtendsThread");
    }
}

public class ExtendDemo {
    public static void main(String[] args) {
        Thread thread = new ExtendsThread();
        thread.start();
    }
}
