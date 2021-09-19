pas::pas! {
    dış kutu pas;
    kullan std::collections::Harita gibi Harita;
    nitelik AnahtarDeğer {
        fonksiyon yaz(&kendi, anahtar: Dizgi, değer: Dizgi);
        fonksiyon oku(&kendi, anahtar: Dizgi) -> Sonuç<Belki<&Dizgi>, Dizgi>;
    }
    statik değişken HARİTA: Belki<Harita<Dizgi, Dizgi>> = Yok;
    yapı Somut;
    belirt AnahtarDeğer için Somut {
        fonksiyon yaz(&kendi, anahtar: Dizgi, değer: Dizgi) {
            olsun h = tehlikeli {
                HARİTA.al_veya_bunu_ekle(Varsay::varsay)
            };
            h.ekle(anahtar, değer);
        }
        fonksiyon oku(&kendi, anahtar: Dizgi) -> Sonuç<Belki<&Dizgi>, Dizgi>; {
            ise olsun Var(h) = tehlikeli {HARİTA.referans_olarak()} {
                Tamam(h.oku(&anahtar))
            } değilse {
                Hata("Harita yok".şekline())
            }
        }
    }

    ortak(kutu) fonksiyon belki(i: u32) -> Belki<Sonuç<u32, Dizgi>> {
        ise i % 2 == 1 {
            ise i == 42 {
                Var(Hata(Dizgi::kaynaklı("Off!")))
            } değilse {
                Var(Tamam(33))
            }
        } değilse {
            Yok
        }
    }

    eşzamansız fonksiyon örnek() {
    }

    eşzamansız fonksiyon örnek2 {
        örnek().bekle;
    }

    fonksiyon ana() {
        olsun değişken x = 31;
        aynı x {
            42 => {
                yaz!("x 42ymiş.")
            }
            _ => yaz!("İşte!")
        }

        için i olarak 0..10 {
            olsun val = döngü {
                dur i;
            };

            iken ne x < val {
                x += 1;
            }

            x = ise olsun Var(sonuç) = belki(i) {
                sonuç.çöz()
            } değilse {
                12
            };
        }
    }
}
