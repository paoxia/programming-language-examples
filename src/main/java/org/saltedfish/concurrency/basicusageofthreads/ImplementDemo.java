package org.saltedfish.concurrency.basicusageofthreads;


/**
 *
 * 实现Runnable接口实现多线程
 */
class ImplementThread implements Runnable {
    public void run() {
        System.out.println("New Thread by ImplementThread");
    }
}



public class ImplementDemo {
    public static void main(String[] args) {
        Thread thread = new Thread(new ImplementThread());
        thread.start();

        //lambda语法
        Thread threadLambda = new Thread(() -> System.out.println("New Thread by ImplementThread Lambda"));
        threadLambda.start();
    }
}
