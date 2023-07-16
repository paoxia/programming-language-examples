package org.saltedfish.concurrency.lockandsynchronization;


class CalculateSyn {
    int count = 0;

    public void add(int n) {
        synchronized (this) {
            count += n;
        }
    }

    public void dec(int n) {
        synchronized (this) {
            count -= n;
        }
    }
}

public class SynchronizedDemo {
    public static void main(String[] args) {
        CalculateSyn calculate = new CalculateSyn();
        Thread threadAdd = new Thread(() -> {
            for (int i = 0; i < 10; i++) {
                calculate.add(1);
                System.out.println("add:" + calculate.count);
                try {
                    Thread.sleep(300);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        });
        Thread threadDec = new Thread(() -> {
            for (int i = 0; i < 10; i++) {
                calculate.dec(1);
                System.out.println("dec:" + calculate.count);
                try {
                    Thread.sleep(300);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        });
        threadAdd.start();
        threadDec.start();
    }
}
