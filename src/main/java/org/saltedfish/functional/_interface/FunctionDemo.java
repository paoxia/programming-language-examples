package org.saltedfish.functional._interface;

import java.util.function.BiFunction;
import java.util.function.BinaryOperator;
import java.util.function.Function;

public class FunctionDemo {
    public static void main(String[] args) {
        Function<Integer, Integer> f = x -> x * 2;
        System.out.println(f.apply(1));

        BiFunction<Integer, Integer, Integer> add = (a, b) ->a + b;
        System.out.println(add.apply(2, 3));

        BinaryOperator<Integer> multiply = (a, b) -> a * b;
        System.out.println(multiply.apply(4, 5));
    }
}
