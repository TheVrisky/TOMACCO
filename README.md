# TOMACCO

> Your Browser. Your Rules. Your Weird Little World.

یک مرورگر دسکتاپ مستقل، متن‌باز و شخصی‌سازی‌پذیر برای Windows 10/11.

ساخته‌شده با **Tauri 2 + Microsoft WebView2 + Rust** — کوچک، متفاوت و متعلق به خودت.

## ویژگی‌های اصلی (نقشه محصول)

| قابلیت | شرح |
|---|---|
| Custom Interface | شخصی‌سازی کامل ظاهر: رنگ، فونت، چیدمان، مخفی‌سازی اجزا |
| Smart Spaces | فضاهای کاری مستقل با رنگ و نشست اختصاصی |
| Command Center | مرکز فرمان سریع با کیبورد |
| Flexible View | تب افقی/عمودی، Sidebar، Split View |
| Dramatic Loading | پیام‌های طنزآمیز حین بارگذاری (قابل غیرفعال‌سازی) |
| Optional Profile Password | رمز Argon2id اختیاری برای هر پروفایل |
| Panic Button | پرش سریع به Safe Profile با یک کلیک یا میانبر |

## فناوری

- Desktop Framework: [Tauri 2](https://v2.tauri.app)
- Web Engine: Microsoft WebView2
- Backend: Rust
- Frontend: HTML / CSS / JavaScript (بدون فریم‌ورک سنگین)
- Config: Browser DNA (JSON مرکزی)

## ساخت (از سورس)

پیش‌نیازها: Rust stable، Node.js 20+، Microsoft WebView2 Runtime (روی Win11 از پیش نصب است).

```bash
npm install
npm run tauri build
```

خروجی نصب‌کننده NSIS در `src-tauri/target/release/bundle/nsis/` قرار می‌گیرد.

## CI

هر push به `main` یک build ویندوزی کامل در [GitHub Actions](../../actions) اجرا می‌کند و نصب‌کننده را به‌عنوان Artifact منتشر می‌کند.

## وضعیت توسعه

مشاهده [docs/PHASE0.md](docs/PHASE0.md) و [docs/PLAN.md](docs/PLAN.md).

## مجوز

MIT
