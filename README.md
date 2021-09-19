# pas

![](logo.png)

**pas** (Turkish for _Rust_) allows to
write Rust programs in Turkish, using Turkish keywords and function names, adapted from the original French implementation [Rouille](https://github.com/bnjbvr/rouille).

Here's an example of Pas usage:

### trait and impl (aka nitelik and belirt)

```rust
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
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## Other languages

- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [oxido](https://github.com/fdschonborn/oxido)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)

## License

[WTFPL](http://www.wtfpl.net/).
