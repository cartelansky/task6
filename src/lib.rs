pub struct Rapor<'a> {
    pub satıs: Vec<String>,
    pub alım: Vec<String>,
    pub envanter: &'a Envanter,
}

impl Rapor<'_> {
    pub fn rapor_goruntule(&self) {
        println!("Satış Bilgileri");
        for i in &self.satıs {
            println!("{i}")
        }
        println!("");
        println!("Alım Bilgileri");
        for i in &self.alım {
            println!("{i}")
        }
        println!("");
        self.envanter.envanter_goruntule();
    }
}

#[derive(Debug)]
pub struct Urun {
    pub adı: String,
    pub acıklama: String,
    pub fiyat: f32,
    pub miktar: u32,
}

#[derive(Debug)]
pub struct Envanter {
    pub urunler: Vec<Urun>,
}

impl Envanter {
    pub fn envanter_goruntule(&self) {
        println!("Envanter Bilgisi");
        for i in &self.urunler {
            println!("{:?}", i)
        }
    }
    pub fn urun_ekle(&mut self, x: Urun) {
        for i in &mut self.urunler {
            if i.adı == x.adı {
                println!("Bu ürün zaten envanterde var");
                return;
            }
        }
        self.urunler.push(x);
    }
    pub fn urun_duzenle(&mut self, x: Urun) {
        for i in &mut self.urunler {
            if i.adı == x.adı {
                *i = x;
                return;
            }
        }
    }
    pub fn urun_sil(&mut self, x: String) {
        let mut index = 0;
        for i in &mut self.urunler {
            if i.adı == x {
                self.urunler.remove(index);
                return;
            }
            index += 1;
        }
    }
}

pub fn satın_alma(x: String, y: u32, z: f32, e: &mut Envanter) -> String {
    println!("{x} ürününden {y} adet ve {z} birim fiyatına alım yapılmıştır");
    for i in &mut e.urunler {
        if i.adı == x {
            i.miktar += y;
            return format!("{z} ürününden {y} adet ve {z} birim fiyatına alım yapılmıştır");
        }
    }
    String::from("Bos deger")
}

pub fn satıs(x: String, y: u32, z: f32, e: &mut Envanter) -> String {
    println!("{x} ürününden {y} adet ve {z} birim fiyatına satış yapılmıştır");
    for i in &mut e.urunler {
        if i.adı == x {
            i.miktar -= y;
            return format!("{x} ürününden {y} adet ve {z} birim fiyatına satış yapılmıştır");
        }
    }
    String::from("Bos deger")
}
