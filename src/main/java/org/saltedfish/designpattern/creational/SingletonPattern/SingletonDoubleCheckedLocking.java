package org.saltedfish.designpattern.creational.SingletonPattern;


/**
 * Lazy初始化
 *
 * 线程安全
 *
 * 使用双锁机制，安全且在多线程情况下能保持高性能
 *
 */

public class SingletonDoubleCheckedLocking {

    private volatile static SingletonDoubleCheckedLocking singleton;

    private SingletonDoubleCheckedLocking () {}

    public static SingletonDoubleCheckedLocking getSingleton() {
        if (singleton == null) {
            synchronized (SingletonDoubleCheckedLocking.class) {
                if (singleton == null) {
                    singleton = new SingletonDoubleCheckedLocking();
                }
            }
        }
        return singleton;
    }
}
