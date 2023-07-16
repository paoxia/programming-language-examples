package io

import (
	"encoding/json"
	"fmt"
	"testing"
)

type JsonHelper struct {
	ParamOne string `json:"param_one"`
}

func TestJson(t *testing.T) {

	from := JsonHelper{ParamOne: "test"}

	str, err := json.Marshal(from)
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	fmt.Println(string(str))

	var to JsonHelper
	json.Unmarshal(str, &to)
	fmt.Println(to.ParamOne)
}
