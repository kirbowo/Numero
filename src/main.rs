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
//fn es_primo(&self) -> bool {}

//fn cantidad_de_digitos(&self) -> u32{}
//    fn invertido(&self) -> u64 {}
}
fn main() {
    let num: Numero = Numero::new(542);
    println!("¿Es par? {}", num.es_par());
}
