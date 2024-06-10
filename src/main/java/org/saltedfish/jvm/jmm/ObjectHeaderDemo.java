package org.saltedfish.jvm.jmm;

import org.openjdk.jol.info.ClassLayout;

public class ObjectHeaderDemo {
    public static void main(String[] args) {
        // jvm detail
        // System.out.println(VM.current().details());

        // System.out.println(VM.current().objectAlignment());

        Object o = new Object();
        System.out.println(ClassLayout.parseInstance(o).toPrintable());
    }
}

class Person {
    int age;
}
