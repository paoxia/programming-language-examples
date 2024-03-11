package org.saltedfish.concurrency.interview;

// 分别打印3 5 和其他的

import java.util.Objects;

public class Multiply3Multiply5Other implements Runnable{
    public static int i = 1;
    @Override
    public void run() {
        synchronized (Multiply3Multiply5Other.class) {
            while (i <= 30) {
                if ((Objects.equals(Thread.currentThread().getName(), "Multiply3") && i % 3 == 0) ||
                        (Objects.equals(Thread.currentThread().getName(), "Multiply5") && i % 5 == 0) ||
                        (Objects.equals(Thread.currentThread().getName(), "Other") && i % 3 != 0 && i % 5 != 0)) {
                    System.out.println(Thread.currentThread().getName() + "$" + i);
                    i++;
                    Multiply3Multiply5Other.class.notifyAll();
                } else {
                    try {
                        Multiply3Multiply5Other.class.wait();
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                }
            }
        }
    }

    public static void main(String[] args) {

        Thread thread1 = new Thread(new Multiply3Multiply5Other(), "Multiply3");
        Thread thread2 = new Thread(new Multiply3Multiply5Other(), "Multiply5");
        Thread thread3 = new Thread(new Multiply3Multiply5Other(), "Other");

        thread1.start();
        thread2.start();
        thread3.start();
    }

}
