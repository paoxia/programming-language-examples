package org.saltedfish.designpattern.behavioral.MediatorPattern;

public class MediatorPatternDemo {
    public static void main(String[] args) {
        User Shon = new User("Shon");
        User Jay = new User("Jay");
        Shon.sendMessage("Ai you");
        Jay.sendMessage("Bu cuo o");
    }
}
