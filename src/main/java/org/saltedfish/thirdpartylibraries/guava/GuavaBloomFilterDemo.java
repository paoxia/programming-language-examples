package org.saltedfish.thirdpartylibraries.guava;

import com.google.common.hash.BloomFilter;
import com.google.common.hash.Funnels;

/**
 *
 * @author SaltedFish
 * @date 2021/1/16
 */
public class GuavaBloomFilterDemo {
    public static void main(String[] args) {
        BloomFilter<Integer> bloomFilter = BloomFilter.create(
                Funnels.integerFunnel(),
                1000,
                0.01
        );

        bloomFilter.put(Integer.valueOf(1));
        bloomFilter.put(Integer.valueOf(2));
        bloomFilter.put(Integer.valueOf(3));

        System.out.println(bloomFilter.mightContain(Integer.valueOf(1)));
        System.out.println(bloomFilter.mightContain(Integer.valueOf(4)));
    }
}
