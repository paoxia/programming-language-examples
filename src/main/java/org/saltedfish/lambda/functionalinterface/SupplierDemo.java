package org.saltedfish.lambda.functionalinterface;

import java.util.function.Supplier;

/**
 * @author SaltedFish
 * @date 2021/2/18
 * 无参数，返回一个结果。
 */
public class SupplierDemo {
    public static void main(String[] args) {
        int a = 1;
        int b = 2;
        Supplier<Integer> supplier = () -> a + b;
        System.out.println(supplier.get());
    }
}
