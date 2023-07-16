package org.saltedfish.designpattern.creational.FactoryPattern;

public class ShapeFactoryDemo {

    public static void main(String[] args) {
        ShapeFactory shapeFactory = new ShapeFactory();

        Shape shape = shapeFactory.getShape("CIRCLE");
        shape.draw();


    }

}
