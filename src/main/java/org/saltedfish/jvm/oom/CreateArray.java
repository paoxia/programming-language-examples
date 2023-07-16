package org.saltedfish.jvm.oom;

/**
 * @author SaltedFish
 * @date 2021/3/24
 */
public class CreateArray {
    public static void main(String[] args) {
        int[] nums = new int[Integer.MAX_VALUE];
    }
}
