fn main() {
    println!("Hello, world!");
    outra_funcao(5, 4);
    let x = cinco();
    let y = 3;
    outra_funcao(x, y);

    println!("O valor da funcao soma_um com parametro 4 e: {}", soma_um(4));
}

fn soma_um(x: i32) -> i32  {
    x + 1
}
fn cinco() -> i32 {
    5
}

fn outra_funcao(x: i32, y: i32) {
    println!("O valor de x e: {}", x);
    println!("O valor de y e: {}", y);
}