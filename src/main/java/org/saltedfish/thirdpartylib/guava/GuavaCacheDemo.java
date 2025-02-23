package org.saltedfish.thirdpartylib.guava;

import java.util.concurrent.TimeUnit;
import com.google.common.cache.Cache;
import com.google.common.cache.CacheBuilder;

/**
 * @author SaltedFish
 */
public class GuavaCacheDemo {
    public static void main(String[] args) {
        Cache<String, Integer> cache = CacheBuilder.newBuilder()
                .maximumSize(100)
                .expireAfterWrite(10, TimeUnit.MINUTES)
                .build();

        cache.put("key1", 1);
        cache.put("key2", 2);
        cache.put("key3", 3);

        System.out.println(cache.getIfPresent("key1"));
        System.out.println(cache.getIfPresent("key5"));
    }
}
