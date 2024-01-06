package org.saltedfish.functional._interface;

public class RunnableDemo {
    public static void main(String[] args) {
        Runnable runnable = ()-> System.out.println("use runnable");
        new Thread(runnable).run();

    }
}
