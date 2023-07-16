package org.saltedfish.designpattern.behavioral.TemplatePattern;

public abstract class Game {
    //共同属性
    abstract void initialize();
    abstract void startPlay();
    abstract void endPlay();

    public final void play() {

        initialize();

        startPlay();

        endPlay();
    }

}
