# Anisub CLI
> **Diğer dillerde okuyun:** [English](README.md)

Türkçe anime altyazı platformu anisub.co için bir CLI aracı.

## Kurulum
Rust ve Cargo'nun kurulu olduğundan emin olun.
```bash
cargo install anisub-cli
```

veya AUR üzerinden kurun:
```bash
yay -S anisub-cli
```

## Kullanım

### Giriş (İsteğe bağlı)

Kişisel token'ınızı kullanarak indirme yapmak için giriş yapabilirsiniz.
[Token almak için buraya tıklayın.](https://anisub.co/ayarlar#api)

```bash
anisub-cli login
```

### Arama ve İndirme

Anime veya altyazı adına göre arama yapabilirsiniz.
```bash
anisub-cli search “bleach”
```

veya `-o` parametresini kullanarak belirli bir klasöre indirebilirsiniz:
```bash
anisub-cli search “bleach” -o ~/Downloads/Subtitles
```

### Kısayollar (Arama Ekranında)
* **Yukarı/Aşağı Ok:** Sonuçlar arasında gezinme
* **Enter:** Seçilen altyazıyı indir
* **Q / ESC:** Çıkış
