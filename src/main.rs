use std::io::stdin;

struct Numero {
    valor: u64
}
impl Numero {
    fn new(valor: u64) -> Self {
        Self { valor }
    }

fn es_par(&self) -> bool {
    self.valor % 2 == 0
}
fn es_primo(&self) -> bool {
    if self.valor < 2 {
        return false;
    }
    for i in 2..=((self.valor as f64).sqrt() as u64) {
        if self.valor % i == 0 {
            return false;
        }
    }
    true
}

fn cantidad_de_digitos(&self) -> u32{
    self.valor.to_string().len() as u32
}
fn invertido(&self) -> u64 {
    //self.valor.to_string().chars().rev().collect::<String>().parse().unwrap_or(0)
    let mut invertido = 0;
    let mut numero = self.valor;
    while numero > 0 {
        invertido = invertido * 10 + numero % 10;
        numero /= 10;
    }
    invertido
}
fn es_par_invertido(&self) -> bool {
    self.invertido() % 2 == 0
}
fn es_primo_invertido(&self) -> bool {
    let invertido = self.invertido();
    if invertido < 2 {
        return false;
    }
    for i in 2..=((invertido as f64).sqrt() as u64) {
        if invertido % i == 0 {
            return false;
        }
    }
    true
}
 
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let num: Numero = Numero::new(input.trim().parse().unwrap_or(0));
    println!("Número ingresado: {}", num.valor);
    println!("Cantidad de dígitos: {}", num.cantidad_de_digitos());
    println!("¿Es par? {}", num.es_par());
    println!("¿Es primo? {}", num.es_primo());
    println!("Número invertido: {}", num.invertido());
    println!("¿Es par el número invertido? {}", num.es_par_invertido());
    println!("¿Es primo el número invertido? {}", num.es_primo_invertido());

}
