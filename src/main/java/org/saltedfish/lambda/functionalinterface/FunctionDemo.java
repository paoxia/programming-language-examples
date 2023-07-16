package org.saltedfish.lambda.functionalinterface;

import java.util.function.BiFunction;

/**
 * @author SaltedFish
 * @date 2021/2/18
 * 一个接受两个输入参数的方法，并且返回一个结果
 */
public class FunctionDemo {
    public static void main(String[] args) {
        BiFunction<Integer, Integer, Integer> biFunction = (a, b) -> a + b;
        System.out.println(biFunction.apply(1, 2));
    }
}
