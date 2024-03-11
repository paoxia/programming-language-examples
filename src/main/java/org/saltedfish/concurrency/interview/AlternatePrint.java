package org.saltedfish.concurrency.interview;

// 从1到100交替打印

import lombok.Synchronized;

public class AlternatePrint implements Runnable{

    public static int num = 1;
    public static int maxNum = 100;

    @Override
    public void run() {
        synchronized (AlternatePrint.class) {
            while (num <= maxNum) {
                System.out.println(Thread.currentThread().getName()+":"+num);
                num++;
                AlternatePrint.class.notify();

                try {
                    if (num < maxNum) {
                        AlternatePrint.class.wait();
                    }
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }

            }
        }

    }

    public static void main(String[] args) {
        Thread thread1 = new Thread(new AlternatePrint(), "thread-1");
        Thread thread2 = new Thread(new AlternatePrint(), "thread-2");

        thread1.start();
        thread2.start();
    }


}
