package org.saltedfish.lambda.functionalinterface;


import java.util.function.Predicate;

/**
 * @author SaltedFish
 * @date 2021/2/18
 * 接受一个输入参数，返回一个布尔值结果。
 */
public class PredicateDemo {
    public static void main(String[] args) {
        int b = 1;
        Predicate<Integer> biPredicate = a -> a > b;
        System.out.println(biPredicate.test(2));
    }
}
