package org.saltedfish.stream;

import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;

public class CreateStreamDemo {
    public static void main(String[] args) {
        /**
         * 使用容器调用stream()方法创建Stream
         */
        List<Integer> list = new ArrayList<>();
        list.add(1);
        long lessThanTwo = list.stream()
                .filter(num -> num <= 2)
                .count();

        /**
         * 使用Stream接口的of方法创建Stream
         */
        Stream<String> nums = Stream.of("one", "two", "three");

        /**
         * 使用Array.stream(array, from, to) 不包括to
         * 其实上一个Stream.of也是调用了此方法
         */
        String[] numsList = new String[]{"one", "two", "three"};
        Arrays.stream(numsList, 0, 3);

        /**
         * 通过generate创建流
         */
        Stream<Double> random = Stream.generate(Math::random);

        /**
         * 通过iterate方法
         */
        Stream<BigInteger> integers = Stream.iterate(BigInteger.ZERO, n -> n.add(BigInteger.ONE));


    }
}
