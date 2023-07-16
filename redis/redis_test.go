package redis

import (
	"log"
	"testing"

	"github.com/gomodule/redigo/redis"
)

func TestRedis(t *testing.T) {
	// connect redis
	conn, err := redis.Dial("tcp", ":6379")
	if err != nil {
		log.Println("connect failed")
	}
	// set value
	setRes, err := conn.Do("SET", "key1", "value1")
	if err != nil {
		log.Println("SET key value failed")
	} else {
		log.Println(setRes)
	}
	// get value
	getRes, err := redis.String(conn.Do("GET", "key1"))
	if err != nil {
		log.Println("GET value failed")
	} else {
		log.Println(getRes)
	}
}
