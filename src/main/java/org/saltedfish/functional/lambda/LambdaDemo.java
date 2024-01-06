package org.saltedfish.functional.lambda;

import java.util.ArrayList;
import java.util.List;

public class LambdaDemo {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);

        list.forEach(num -> System.out.println(num));
        list.forEach(System.out::println);
    }
}
