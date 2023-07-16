package org.saltedfish.concurrency.basicusageofthreads;

/**
 * 有些线程执行一些诸如定时任务等
 * 当没有线程运行时必须要停止
 * 可将其设置成守护线程
 */

class TickThread implements Runnable {
    @Override
    public void run() {
        while (true) {
            try {
                Thread.sleep(1000);
                System.out.println("Daemon Thread Print");
            } catch (InterruptedException e) {
                e.printStackTrace();
            }

        }
    }
}

public class DaemonDemo {
    public static void main(String[] args) throws InterruptedException {
        Thread thread = new Thread(new TickThread());
        thread.setDaemon(true);
        thread.start();
        Thread.sleep(5000);
    }
}
