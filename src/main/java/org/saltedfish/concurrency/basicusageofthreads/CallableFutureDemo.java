package org.saltedfish.concurrency.basicusageofthreads;

import java.util.concurrent.*;

/**
 * 带返回值的线程
 */

class CallableFutureTask implements Callable<Integer> {
    @Override
    public Integer call() {
        return 720;
    }
}

public class CallableFutureDemo {
    public static void main(String[] args) throws ExecutionException, InterruptedException {
        ExecutorService executorService = Executors.newFixedThreadPool(3);
        Future<Integer> res = executorService.submit(new CallableFutureTask());
        System.out.println(res.get());
    }
}
