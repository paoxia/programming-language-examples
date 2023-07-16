package org.saltedfish.designpattern.behavioral.TemplatePattern;

public class Football extends Game{

    @Override
    void initialize() {
        System.out.println("Football Game Initialize");
    }

    @Override
    void startPlay() {
        System.out.println("Football Game Started");
    }

    @Override
    void endPlay() {
        System.out.println("Football Game Finished");
    }
}
