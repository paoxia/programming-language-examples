package org.saltedfish.functional._interface;

import java.util.function.Function;

public class FunctionDemo {
    public static void main(String[] args) {
        Function<Integer, Integer> f = x -> x * 2;
        System.out.println(f.apply(1));
    }
}
