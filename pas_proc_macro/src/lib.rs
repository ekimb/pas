use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Hata" => "Err",
        "Tamam" => "Ok",
        "Dizgi" => "String",
        "Harita" => "HashMap",
        "Varsay" => "Default",
        "Sorun" => "Error",
        "Belki" => "Option",
        "Var" => "Some",
        "Yok" => "None",
        "Sonuç" | "Sonuc" => "Result",
        "Kendi" => "Self",
        "yaz" => "println",
        "dur" => "break",
        "eşzamansız" | "eszamansiz" => "async",
        "bekle" => "await",
        "döngü" | "dongu" => "loop",
        "taşı" | "tasi" => "move",
        "kutu" => "crate",
        "ulaşılamaz_kod" | "ulasilamaz_kod" => "unreachable_code",
        "gibi" => "as",
        "sabit" => "const",
        "nitelik" => "trait",
        "tehlikeli" => "unsafe",
        "olarak" => "in",
        "kaynaklı" | "kaynakli" => "from",
        "dinamik" => "dyn",
        "çöz" | "coz" => "unwrap",
        "varsay" => "default",
        "referans_olarak" => "as_ref",
        "gç" / "gc" => "io",
        "dış" | "dis" => "extern",
        "yanlış" | "yanlis" => "false",
        "işlev" | "islev" | "fonksiyon" => "fn",
        "süper" | "super" => "super",
        "ekle" => "insert",
        "al" => "get",
        "izin" => "allow",
        "panik" | "kork" => "panic",
        "modül" | "modul" | "birim" => "mod",
        "degisken" | "değişken" => "mut",
        "yeni" => "new",
        "nerede" => "where",
        "icin" | "için" => "for",
        "al_veya_bunu_ekle" => "get_or_insert_with",
        "ana" => "main",
        "ortak" => "pub",
        "ne" => None?,
        "dondur" | "döndür" => "return",
        "belirt" => "impl",
        "ref" | "referans" => "ref",
        "ayni" | "aynı" => "match",
        "ise" => "if",
        "degilse" | "değilse" => "else",
        "kendi" => "self",
        "olsun" => "let",
        "statik" => "static",
        "yapi" | "yapı" => "struct",
        "bekle" => "expect",
        "iken" => "while",
        "kullan" => "use",
        "sekline" | "şekline" => "into",
        "dogru" | "doğru" => "true",
        "say" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn pas(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
