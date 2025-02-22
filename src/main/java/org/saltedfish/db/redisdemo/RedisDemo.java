package org.saltedfish.db.redisdemo;

import redis.clients.jedis.Jedis;
import redis.clients.jedis.JedisPool;

public class RedisDemo {

    public static void main(String[] args) {
        // Jedis pool

        try {
            JedisPool pool = new JedisPool("localhost", 6379);
            Jedis jedis = pool.getResource();
            if (jedis != null) {
                jedis.set("clientName", "Jedis");
                System.out.println(jedis.get("clientName"));
            }
        } catch (Exception e) {

        }

    }
}
