package org.saltedfish.concurrency.conditionwaitnotify;

import java.util.LinkedList;
import java.util.Queue;

class ProducerAndConsumerWaitNotify {

    private Queue<Integer> queue = new LinkedList<>();

    public synchronized void producer(Integer i) {
        queue.add(i);
        this.notifyAll();
    }

    public synchronized void consumer() throws InterruptedException {
        while (queue.size() == 0) {
            this.wait();
        }
        System.out.println("consumer:" + queue.remove());
    }

}


public class WaitNotify {
    public static void main(String[] args) {
        ProducerAndConsumerWaitNotify producerAndCosumer = new ProducerAndConsumerWaitNotify();
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
