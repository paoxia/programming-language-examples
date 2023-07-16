package org.saltedfish.designpattern.behavioral.ObserverPattern;

import java.util.ArrayList;
import java.util.List;

public class Subject {

    //保存观察者的数据结构
    private List<Observer> observers = new ArrayList<Observer>();

    private int state;

    public int getState() {
        return state;
    }

    //设置状态 通知所有观察者
    public void setState(int state) {
        this.state = state;
        notifyAllObservers();
    }

    //在观察者的构造函数种使用
    public void attach(Observer observer) {
        observers.add(observer);
    }

    public void notifyAllObservers() {
        for (Observer observer : observers) {
            observer.update();
        }
    }
}
