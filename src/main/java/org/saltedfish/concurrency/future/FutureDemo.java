package org.saltedfish.concurrency.future;

import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.FutureTask;


class FutureThread implements Callable<Integer> {

    @Override
    public Integer call() throws Exception {
        return 0;
    }
}

public class FutureDemo {


    public static void main(String[] args) throws ExecutionException, InterruptedException {
        ExecutorService executorService = Executors.newFixedThreadPool(5);
        Callable<Integer> callable = new Callable<Integer>() {
            @Override
            public Integer call() throws Exception {
                return -1;
            }
        };

        Future<Integer> future = executorService.submit(callable);

        Integer res = future.get();
        System.out.println(res);
        executorService.shutdown();
        FutureTask<Integer> future1 = new FutureTask<>(callable);
        Thread t1 = new Thread(future1, "t1");
        t1.start();
        System.out.println(future1.get());
    }
}
