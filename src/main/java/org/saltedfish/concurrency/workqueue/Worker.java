package org.saltedfish.concurrency.workqueue;


/**
 * 工作线程
 */
public class Worker {

    /**
     * 工作任务
     *
     * @param num 数字
     */
    public static Load work(int num) {
        System.out.println("worker-" + num);

        return Load.builder().num(num).build();
    }
}
