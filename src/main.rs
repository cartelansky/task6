use std::process::exit;

use task6::{satın_alma, satıs};
use task6::{Envanter, Rapor, Urun};

fn main() {
    // yönetici şifresi fb1907 atanmıştır.
    let sifre = "fb1907";
    let bot = Urun {
        adı: "Bot".to_string(),
        acıklama: "Kıslık botlar".to_string(),
        miktar: 10,
        fiyat: 25.6,
    };

    let polar = Urun {
        adı: String::from("Polar"),
        acıklama: String::from("Sportif polarlar"),
        fiyat: 17.3,
        miktar: 5,
    };

    let gozluk = Urun {
        adı: "Gozluk".to_string(),
        acıklama: "Gunes gozlukleri".to_string(),
        fiyat: 11.8,
        miktar: 85,
    };

    let mut envanter = Envanter {
        urunler: vec![bot, gozluk, polar],
    };
    let mut rapor = Rapor {
        satıs: vec![],
        alım: vec![],
        envanter: &envanter,
    };
    println!("merhaba lütfen yönetici şifrenizi giriniz: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == sifre {
        println!("Envanter yönetim sistemine hoşgeldiniz\n");
    } else {
        panic!("Yanlış şifre, sistemden atıldınız");
    }
    println!("Yönetici olarak yapabileceğiniz işlemler şunlardır:\nEnvarteri görüntüleme(0)\nEnvantere ürün ekleme(1)\nEnvanterdeki ürünü düzenleme(2)\nEnvanterden ürün silme(3)\n\nSatış işlemi(4)\nSatın alma işlemi(5)\nRaporlama işlemi(6)\nÇıkış(7)\n");

    println!("Lütfen yapacağınız işlemin kodunu giriniz:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("");

    match input.trim() {
        "0" => envanter.envanter_goruntule(),
        "1" => {
            println!("Eklemek istediğiniz ürünün adını, açıklamasını, fiyatını ve miktarını giriniz.Fiyat yazarken ondalık sayı için nokta kullanın.");
            println!("Ürünün adı:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            println!("Ürünün açıklaması:");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();
            println!("Ürünün fiyatı:");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            println!("Ürünün miktarı:");
            let mut input3 = String::new();
            std::io::stdin().read_line(&mut input3).unwrap();
            let x = Urun {
                adı: input,
                acıklama: input1,
                fiyat: input2.trim().parse::<f32>().unwrap(),
                miktar: input3.trim().parse::<u32>().unwrap(),
            };
            envanter.urun_ekle(x);
        }
        "2" => {
            println!("Düzenlemek istediğiniz ürünün adını, açıklamasını, fiyatını ve miktarını giriniz.Fiyat yazarken ondalık sayı için nokta kullanın.");
            println!("Ürünün adı:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            println!("Ürünün açıklaması:");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();
            println!("Ürünün fiyatı:");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            println!("Ürünün miktarı:");
            let mut input3 = String::new();
            std::io::stdin().read_line(&mut input3).unwrap();
            let x = Urun {
                adı: input,
                acıklama: input1,
                fiyat: input2.trim().parse::<f32>().unwrap(),
                miktar: input3.trim().parse::<u32>().unwrap(),
            };
            envanter.urun_duzenle(x);
        }
        "3" => {
            println!("Silmek istediğiniz ürünün adını giriniz");
            println!("Ürünün adı:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            envanter.urun_sil(input);
        }
        "4" => {
            println!("Satış yapmak istediğiniz ürünün adını yazınız");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("Kaç adet satış yapılacağını yazınız");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();

            println!("Satış fiyatını yazınız");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            rapor.satıs.push(satıs(
                input.trim().parse().unwrap(),
                input1.trim().parse::<u32>().unwrap(),
                input2.trim().parse::<f32>().unwrap(),
                &mut envanter,
            ))
        }
        "5" => {
            println!("Satın almak istediğiniz ürünün adını yazınız");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            println!("Kaç adet satın alım yapılacağını yazınız");
            let mut input1 = String::new();
            std::io::stdin().read_line(&mut input1).unwrap();

            println!("Alım fiyatını yazınız");
            let mut input2 = String::new();
            std::io::stdin().read_line(&mut input2).unwrap();
            rapor.alım.push(satın_alma(
                input.trim().parse().unwrap(),
                input1.trim().parse::<u32>().unwrap(),
                input2.trim().parse::<f32>().unwrap(),
                &mut envanter,
            ))
        }
        "6" => {
            rapor.rapor_goruntule();
        }
        "7" => exit(0),
        _ => (),
    }
}
