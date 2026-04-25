package encoding

import (
	"encoding/xml"
	"fmt"
	"testing"
)

// Book 书籍结构体
type Book struct {
	XMLName xml.Name `xml:"book"`
	Title   string   `xml:"title"`
	Author  string   `xml:"author"`
	Price   float64  `xml:"price"`
}

// TestXmlMarshal XML序列化
func TestXmlMarshal(t *testing.T) {
	book := Book{
		Title:  "Go语言圣经",
		Author: "Alan A.A.Donovan",
		Price:  99.99,
	}

	data, err := xml.MarshalIndent(book, "", "  ")
	if err != nil {
		fmt.Printf("Marshal error: %v\n", err)
		return
	}
	fmt.Printf("XML:\n%s\n", xml.Header+string(data))
}

// TestXmlUnmarshal XML反序列化
func TestXmlUnmarshal(t *testing.T) {
	xmlStr := `
<book>
  <title>Go语言圣经</title>
  <author>Alan A.A.Donovan</author>
  <price>99.99</price>
</book>`

	var book Book
	err := xml.Unmarshal([]byte(xmlStr), &book)
	if err != nil {
		fmt.Printf("Unmarshal error: %v\n", err)
		return
	}
	fmt.Printf("Book: %+v\n", book)
}
