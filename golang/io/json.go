package main

import (
	"encoding/json"
	"fmt"
)

type JsonHelper struct {
	ParamOne string `json:"param_one"`
}

func main() {

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
