// 定义接口,用于限制person对象的具体属性
export interface PersonInter {
    id:string,
    name:string,
    age:number
}

// export type Persons = Array<PersonInter>
export type Persons = PersonInter[]