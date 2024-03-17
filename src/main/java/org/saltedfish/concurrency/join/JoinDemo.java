package org.saltedfish.concurrency.join;

public class JoinDemo {

    public class ThreadA implements Runnable {

        @Override
        public void run() {
            System.out.println(Thread.currentThread().getName() + "运行结束");
        }
    }

    public static void main(String[] args) throws InterruptedException {
        Thread thread = new Thread(new JoinDemo().new ThreadA(), "thread-A");
        thread.start();
        thread.join();
        System.out.println("如果是join会等ThreadA结束再结束");
    }

}
