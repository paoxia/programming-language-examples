package org.saltedfish.designpattern.creational.SingletonPattern;

/**
 *
 * Lazy初始化
 * 多线程不安全
 *
 * 不支持多线程，因为没有加锁。再多线程模式下可能会产生多个instance
 *
 */
public class SingletonLazyNotThreadSafe {

    private static SingletonLazyNotThreadSafe instance;

    private SingletonLazyNotThreadSafe(){}

    public static SingletonLazyNotThreadSafe getInstance() {
        if (instance == null) {
            instance = new SingletonLazyNotThreadSafe();
        }
        return instance;
    }
}
