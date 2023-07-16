package org.saltedfish.designpattern.behavioral.ObserverPattern;

public class ObserverPatternDemo {

    static int test = 1;

    public static void main(String[] args) {
        Subject subject = new Subject();
        new BinaryObserver(subject);
        new OctalObserver(subject);

        subject.setState(15);
    }
}
