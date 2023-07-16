package org.saltedfish.designpattern.structural.ProxyPattern;

public class ProxyPatternDemo {
    public static void main(String[] args) {
        Image image = new ProxyImage("test.png");

        image.display();


    }
}
