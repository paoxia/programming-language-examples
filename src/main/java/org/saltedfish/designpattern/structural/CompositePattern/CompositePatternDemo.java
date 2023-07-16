package org.saltedfish.designpattern.structural.CompositePattern;

public class CompositePatternDemo {
    public static void main(String[] args) {
        Employee CEO = new Employee("Shon", "CEO", 30000);
        Employee headSale = new Employee("Jay", "Head Salse", 2000);
        Employee salesMan = new Employee("Marry", "Sales Man", 200);
        CEO.add(headSale);
        headSale.add(salesMan);
        for (Employee headEmployee : CEO.getSubordinates()) {
            for (Employee employee : headEmployee.getSubordinates()) {
                System.out.println(employee);
            }
        }
    }
}
