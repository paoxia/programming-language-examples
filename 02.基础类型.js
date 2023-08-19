// 任意类型
var a = 'any type';
console.log("a=", a);
// 字符串类型
var b = 'string';
console.log("b=", b);
// 布尔类型
var c = false;
console.log("c=", c);
// 数组类型
var d = [1, 2];
console.log("d=", d);
// 元组类型
var e;
e = ['hello', 1];
console.log("e0=", e[0], "e1=", e[1]);
// 枚举类型
var Color;
(function (Color) {
    Color[Color["RED"] = 0] = "RED";
    Color[Color["GREEN"] = 1] = "GREEN";
    Color[Color["BlUE"] = 2] = "BlUE";
})(Color || (Color = {}));
var f = Color.BlUE;
console.log(f);
// void 标识空类型
// null 标识对象值缺失
// undefined 用于初始化变量为一个未定义的值
// nerver是其他类型包括 null 和 undefined）的子类型，代表从不会出现的值。
