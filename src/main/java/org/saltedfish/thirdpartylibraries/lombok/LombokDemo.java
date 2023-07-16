package org.saltedfish.thirdpartylibraries.lombok;


import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.Value;

/**
 * @Setter  提供setter方法
 * @Getter  提供getter方法
 * @AllArgsConstructor 提供全参构造函数
 * @NoArgsConstructor 提供无参构造函数
 * @EqualsAndHashCode 提供equals和hashcode方法
 * @Data 提供setter方法、getter方法、equals和hashcode方法
 * @Builder 构建者模式
 */
@Setter
@Getter
@AllArgsConstructor
@NoArgsConstructor
@EqualsAndHashCode
@Data
@Builder
class Person {
    private int age;
    private String name;
}

/**
 * @Value 有参构造方法 所有属性变成final
 */
@Value
class dog {
    private int age;
    private String name;
}

public class LombokDemo {
    public static void main(String[] args) {
        Person person = new Person(12, "ShonYu");
        System.out.println(person.toString());

        //使用@Builder后
        person = Person.builder()
                .age(24)
                .name("ShonYu")
                .build();
        System.out.println(person.toString());
    }
}
