struct coin {
    name: String,
    miktar: f64,
}

impl coin {
    fn yeni(name: &str, miktar: f64) -> Self {
        coin {
            name: String::from(name),
            miktar,
        }
    }
}

fn main() {
    // x adındaki kripto para
    let mut x_crypto = coin::yeni("X Coin", 10.0);

    // t adındaki kripto para
    let mut t_crypto = coin::yeni("T Coin", 5.0);

    println!("Swap öncesi durum:");
    println!("X Coin: {} adet", x_crypto.miktar);
    println!("T Coin: {} adet", t_crypto.miktar);

    // Swap işlemi
    swap_crypto(&mut x_crypto, &mut t_crypto);

    println!("\nSon durum:");
    println!("X Coin: {} adet", x_crypto.miktar);
    println!("T Coin: {} adet", t_crypto.miktar);
}

fn swap_crypto(x: &mut coin, t: &mut coin) {
    // X Coin ve T Coin'in miktarlarını değiştir
    let temp_miktar = x.miktar;
    x.miktar = t.miktar;
    t.miktar = temp_miktar;
}