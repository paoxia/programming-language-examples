package org.saltedfish.designpattern.creational.SingletonPattern;

/**
 *
 * Lazy初始化
 *
 * 多线程安全
 *
 * 这种方式具备很好的 lazy loading，能够在多线程中很好的工作，但是，效率很低，99% 情况下不需要同步。
 * 优点：第一次调用才初始化，避免内存浪费。
 * 缺点：必须加锁 synchronized 才能保证单例，但加锁会影响效率。
 *
 */
public class SingletonLazyThreadSafe {

    private static SingletonLazyThreadSafe instance;


    private SingletonLazyThreadSafe() {}

    public static synchronized SingletonLazyThreadSafe getInstance() {
        if (instance == null) {
            instance = new SingletonLazyThreadSafe();
        }
        return instance;
    }

}
