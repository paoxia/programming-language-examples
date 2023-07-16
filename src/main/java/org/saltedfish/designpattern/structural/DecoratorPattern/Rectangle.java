package org.saltedfish.designpattern.structural.DecoratorPattern;

public class Rectangle implements Shape {

    @Override
    public void draw() {
        System.out.println("Shape:Rectangle");
    }

}
