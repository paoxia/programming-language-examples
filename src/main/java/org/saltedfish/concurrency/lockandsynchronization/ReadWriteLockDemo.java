package org.saltedfish.concurrency.lockandsynchronization;

import lombok.Getter;
import lombok.Setter;

import java.util.Arrays;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

/**
 * 例子参考于
 * https://www.liaoxuefeng.com/wiki/1252599548343744/1306581002092578
 */

@Getter
@Setter
class ReadWriteLockArray {
    private final ReentrantReadWriteLock rwlock = new ReentrantReadWriteLock();
    private final Lock wlock = rwlock.writeLock();
    private final Lock rlock = rwlock.readLock();

    private int[] num = new int[6];

    public void inc(int index) {
        wlock.lock();
        try {
            num[index] += 1;
        } finally {
            wlock.unlock();
        }
    }

    public void get() {
        rlock.lock();
        try {
            System.out.println(Arrays.toString(num));
        } finally {
            rlock.unlock();
        }
    }

}

public class ReadWriteLockDemo {
    public static void main(String[] args) throws InterruptedException {
        ReadWriteLockArray array = new ReadWriteLockArray();
        Thread incThread = new Thread(() -> {
            for (int i = 0; i < array.getNum().length; ++i) {
                array.inc(i);
            }
        });
        for (int i = 0; i < array.getNum().length; ++i) {
            Thread getThread = new Thread(() -> array.get());
            getThread.start();
            Thread.sleep(500);
        }
        incThread.start();
    }
}
