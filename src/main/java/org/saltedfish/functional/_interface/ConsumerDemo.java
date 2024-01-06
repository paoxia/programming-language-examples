package org.saltedfish.functional._interface;

import java.util.ArrayList;
import java.util.List;
import java.util.function.BiConsumer;
import java.util.function.Consumer;

public class ConsumerDemo {
    public static void main(String[] args) {
        Consumer<Integer> consumer = num -> System.out.println(num);

        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);

        list.forEach(consumer);

        BiConsumer<String, Integer> biConsumer = (name, age) -> System.out.println("name:"+ name + "age:" + age);
        biConsumer.accept("John", 30);
    }
}
