package org.saltedfish.lambda.functionalinterface;

@FunctionalInterface
interface SayHello
{
    void sayMessage(String message);
}

public class GreetingService {
    public static void main(String[] args) {
        SayHello sayHello = message -> {
            System.out.println("Say Hello " + message);
        };
        sayHello.sayMessage("Ni hao");
    }
}
