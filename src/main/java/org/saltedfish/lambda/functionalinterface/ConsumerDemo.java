package org.saltedfish.lambda.functionalinterface;

import java.util.function.Consumer;

/**
 * @author SaltedFish
 * @date 2021/2/18
 * 接受一个输入参数并且无返回的操作
 */
public class ConsumerDemo {
    public static void main(String[] args) {
        int b = 2;
        Consumer<Integer> consumer = (a) -> System.out.println(a + b);
        consumer.accept(1);
    }
}
