package org.saltedfish.datastructure;

import java.util.Random;

public class SkipList {
    private static final int MAX_LEVEL = 16;
    private int levelCount = 1;
    private Node head = new Node();  // 定义头节点
    private Random random = new Random();

    // 节点类
    static class Node {
        int value = -1;
        Node[] forward = new Node[MAX_LEVEL];
        int maxLevel = 0;

        @Override
        public String toString() {
            return String.valueOf(value);
        }
    }

    // 查找操作
    public Node find(int value) {
        Node p = head;
        for (int i = levelCount - 1; i >= 0; i--) {
            while (p.forward[i] != null && p.forward[i].value < value) {
                p = p.forward[i];
            }
        }
        if (p.forward[0] != null && p.forward[0].value == value) {
            return p.forward[0];
        }
        return null;
    }

    // 插入操作
    public void insert(int value) {
        int level = randomLevel();
        Node newNode = new Node();
        newNode.value = value;
        newNode.maxLevel = level;
        Node[] update = new Node[level];
        for (int i = 0; i < level; i++) {
            update[i] = head;
        }

        Node p = head;
        for (int i = level - 1; i >= 0; i--) {
            while (p.forward[i] != null && p.forward[i].value < value) {
                p = p.forward[i];
            }
            update[i] = p;
        }

        for (int i = 0; i < level; i++) {
            newNode.forward[i] = update[i].forward[i];
            update[i].forward[i] = newNode;
        }

        if (levelCount < level) levelCount = level;
    }

    // 删除操作
    public void delete(int value) {
        Node[] update = new Node[levelCount];
        Node p = head;
        for (int i = levelCount - 1; i >= 0; i--) {
            while (p.forward[i] != null && p.forward[i].value < value) {
                p = p.forward[i];
            }
            update[i] = p;
        }

        if (p.forward[0] != null && p.forward[0].value == value) {
            for (int i = 0; i < levelCount; i++) {
                if (update[i].forward[i] != p.forward[0]) break;
                update[i].forward[i] = p.forward[0].forward[i];
            }
        }
        while (levelCount > 1 && head.forward[levelCount - 1] == null) {
            levelCount--;
        }
    }

    // 随机生成索引层级
    private int randomLevel() {
        int level = 1;
        for (int i = 1; i < MAX_LEVEL; i++) {
            if (random.nextInt() % 2 == 1) {
                level++;
            }
        }
        return level;
    }

    // 打印跳表
    public void printAll() {
        Node p = head;
        while (p.forward[0] != null) {
            System.out.print(p.forward[0] + " ");
            p = p.forward[0];
        }
        System.out.println();
    }

    public static void main(String[] args) {
        SkipList list = new SkipList();
        list.insert(1);
        list.insert(3);
        list.insert(7);
        list.insert(8);
        list.insert(9);
        list.insert(19);
        list.insert(21);
        list.insert(25);
        list.printAll();
        list.delete(3);
        list.printAll();
    }
}

