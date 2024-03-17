package org.saltedfish.concurrency.threadlocal;

public class ThreadlocalDemo {

    class ThreadA implements  Runnable {
        private ThreadLocal<Integer> threadLocal;

        ThreadA(ThreadLocal<Integer> threadLocal) {
            this.threadLocal = threadLocal;
        }
        @Override
        public void run() {
            threadLocal.set(123);

            System.out.println("ThreadA" + threadLocal.get());
        }
    }

    class ThreadB implements Runnable {
        private ThreadLocal<Integer> threadLocal;

        ThreadB(ThreadLocal<Integer> threadLocal) {
            this.threadLocal = threadLocal;
        }
        @Override
        public void run() {
            threadLocal.set(345);

            System.out.println("ThreadB" + threadLocal.get());
        }
    }


    public static void main(String[] args) {
        ThreadLocal<Integer> threadLocal = new ThreadLocal<>();
        Thread threadA = new Thread(new ThreadlocalDemo().new ThreadA(threadLocal));
        Thread threadB = new Thread(new ThreadlocalDemo().new ThreadB(threadLocal));

        threadA.start();
        threadB.start();
    }
}
