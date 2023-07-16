package org.saltedfish.stream;


import java.util.ArrayList;
import java.util.List;

public class StreamDemo {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);
        long lessThanTwo = list.stream()
                .filter(num -> num <= 2)
                .count();

    }
}
