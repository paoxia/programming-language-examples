package org.saltedfish.concurrency.lockandsynchronization;

import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

class CalculateLock {

    int count = 0;

    /**
     * 可设置为公平锁，默认为非公平
     * private Lock calLock = new ReentrantLock(true);
     */
    private Lock calLock = new ReentrantLock();

    public void add(int num) {
        calLock.lock();
        try {
            count += num;
        } finally {
            calLock.unlock();
        }
    }

    public void dec(int num) {
        calLock.lock();
        try {
            count -= num;
        } finally {
            calLock.unlock();
        }
    }
}

public class LockDemo {
    public static void main(String[] args) {
        CalculateLock calculate = new CalculateLock();
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
