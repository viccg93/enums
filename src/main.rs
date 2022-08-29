//Enums es una abreviacion de enumeraciones
//la funcion de un enum es poder definir un tipo de acuerdo a sus posibles variantes

//definiciond e un enum, indicamos con (tipo) el type de cada variacion
enum IpAddrsKind {
    V4,
    V6(String),
}

//los enums se pueden tambien indicar como params
fn route(ip_kind: IpAddrsKind){}

//si quisieramos almacenar direcciones ip podriamos pensar en un struct como el siguiente
struct IpAddress {
    address: String,
    kind: IpAddrsKind,
}

//pero es redundante, ya que los enum pueden almacenar valores, ver main


fn main() {
    //llamada a funcion con parametro enum (con expresion y variable)
    route(IpAddrsKind::V4);

    // comparacion entre struct y enum

    let ip_home = IpAddress{
        address: String::from("127.0.0.1"),
        kind: IpAddrsKind::V4, //se puede usar de esta manera por que no asociamos uno o varios types
    };

    let loopback = IpAddrsKind::V6(String::from("::1")); //mas claro y directo con enum

    //dentro de los enums se puede poner cualquier valor, numericos, string, structs e incluso enums

    //Structs con 4 variantes que tienen diferentes tipos
    #[derive(Debug)]
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(u8,u8,u8),
    }

    //en caso de querer hacer lo mismo con structs, se tendrian que crear 4 structs
    //ademas de que en caso que se quiera tener una funcion en comun a todas las variantes
    //se tendria que crear para cada struct, contrario a enum
    //se crea un bloque impl para ese enum y la funcion aplica todas las variantes
    //self toma el valor de la variante

    impl Message {
        fn call (&self){
            dbg!(&self);
        }
    }

    let m = Message::Write(String::from("Rust"));
    m.call();

    //Entre los Enums mas utiles de Rust se encuetra Option
    //Option es un enum que es parte del preludio
    //Rust no cuenta con el tipo null, debido a algunos problemas con el uso de dicho valor
    //el concepto que rescata Rust de null es: null significa que el valor esta ausente o no es valido

    //la declaracion de enum en la libreria standard es similar a la siguiente:__rust_force_expr!

    //enum Option<T>{
    //    None,
    //    Some(T),
    //}

    let some_number = Some(5);
    let some_char = Some('a');

    //para almacenar la variante None debemos indicar el tipo explicitamente
    //ya que no se puede infereir
    let absent_number: Option<u32> = None;

    //La asignacion de las variantes de un enum se realiza como variante, no como valor
    //es decir que al asignar Some(5), no es lo mismo que 5
    //ya que Some(5) tiene el enum y el valor, este valor se debe de obtener

    let x = 5;
    let y = Some(5);

    //el compilador no sabe como sumar u32 a Option<u32>
    //x+y; //no compila

}
