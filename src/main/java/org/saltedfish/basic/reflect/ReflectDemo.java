package org.saltedfish.basic.reflect;


class Person {
    private String name = "zhangsan";
    private int age = 24;

    public void say(String word) {
        System.out.println(word);
    }
}

/**
 * reflect
 */
public class ReflectDemo {
    public static void main(String[] args) throws IllegalAccessException, InstantiationException {
        //Object中的getClass()会获取一个Class类型的实例
        Person person = new Person();
        Class c1 = person.getClass();
        System.out.println(person.getClass().getName());

        //通过静态方法forName获得类名对应的Class对象
        try {
            String className = "org.saltedfish.basic.reflect.Person";
            Class c2 = Class.forName(className);
            System.out.println(c2.getName());
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFound: " + e.getMessage());
        }

        //通过T.class获取Class对象
        Class c3 = Person.class;
        System.out.println(c3.getName());

        //比较class是否相等
        System.out.println(person.getClass() == Person.class);

        //使用newInstance()动态创建类的实例
        Person person1 = person.getClass().newInstance();
        System.out.println(person1);


    }
}
