package org.saltedfish.concurrency.conditionwaitnotify;

import java.util.LinkedList;
import java.util.Queue;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

class ProducerAndConsumerCondition {

    private final Lock lock = new ReentrantLock();

    private final Condition condition = lock.newCondition();

    private Queue<Integer> queue = new LinkedList<>();

    public void producer(Integer i) {
        lock.lock();
        try {
            queue.add(i);
            condition.signalAll();
        } finally {
            lock.unlock();
        }
    }

    public void consumer() throws InterruptedException {
        lock.lock();
        try {
            while (queue.size() == 0) {
                condition.await();
            }
            System.out.println("consumer:" + queue.remove());
        } finally {
            lock.unlock();
        }
    }

}


public class ConditionDemo {
    public static void main(String[] args) {
        ProducerAndConsumerCondition producerAndCosumer = new ProducerAndConsumerCondition();
        Thread producer = new Thread(() -> {
            for (int i = 0; i < 10; i++) {
                System.out.println("produce:" + i);
                producerAndCosumer.producer(i);
            }
        });

        for (int i = 0; i < 10; ++i) {
            Thread consumer = new Thread(() -> {
                try {
                    producerAndCosumer.consumer();
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            });
            consumer.start();
        }
        producer.start();
    }
}
