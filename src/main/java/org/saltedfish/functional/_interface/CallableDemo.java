package org.saltedfish.functional._interface;

import java.util.concurrent.*;

public class CallableDemo {
    public static void main(String[] args) throws ExecutionException, InterruptedException {

        Callable<String> callable = () -> "Lambda Callable";
        ExecutorService executorService = Executors.newFixedThreadPool(3);
        Future<String> future = executorService.submit(callable);
        System.out.println(future.get());
    }

}

