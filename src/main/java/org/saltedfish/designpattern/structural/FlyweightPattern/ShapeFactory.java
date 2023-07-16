package org.saltedfish.designpattern.structural.FlyweightPattern;

import java.util.HashMap;

public class ShapeFactory {
    private static final HashMap<String, Shape> circleMap = new HashMap<>();

    //若已经存在对应的对象则直接返回
    public static Shape getCircle (String color) {
        Circle circle = (Circle)circleMap.get(color);
        if (circle == null) {
            circle = new Circle(color);
            circleMap.put(color, circle);
            System.out.println("Creating circle of color: " + color);
        }
        return circle;
    }
}
