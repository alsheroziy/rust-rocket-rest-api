<!-- @format -->

# Shop Rocket API 

Bu loyiha **Rust** dasturlash tili va **Rocket** freymvorki yordamida yozilgan zamonaviy REST API hisoblanadi. Loyiha "Clean Architecture" tamoyillari asosida tuzilgan va barcha "Senior" darajadagi talablarga javob beradi.

## 🚀 Texnologiyalar

- **Rust**: Asosiy dasturlash tili.
- **Rocket**: Web framework.
- **SQLx (SQLite)**: Asinxron ma'lumotlar bazasi drayveri.
- **Argon2**: Parollarni xavfsiz xeshlash.
- **JWT (JSON Web Token)**: Autentifikatsiya va avtorizatsiya.
- **Utoipa (Swagger UI)**: API hujjatlari.
- **Tokio**: Asinxron runtime.
- **Serde**: Ma'lumotlarni seriyalash/deseriyalash.

## 🏗 Loyiha Tuzilmasi

Loyiha quyidagi qatlamlarga (layers) ajratilgan:

- **`src/api`**: HTTP so'rovlarini qabul qiluvchi handlerlar (Controller).
- **`src/domain`**: Asosiy biznes ma'lumotlar modellari (Entities, DTOs).
- **`src/repository`**: Ma'lumotlar bazasi bilan ishlash logikasi (DAO).
- **`src/infrastructure`**: Yordamchi kodlar (Auth, Config, Xavfsizlik).
- **`migrations/`**: Bazani boshqarish uchun SQL skriptlar.

## 🛠 O'rnatish va Ishga Tushirish

1.  **Talablar**:

    - Rust o'rnatilgan bo'lishi kerak (`cargo`).

2.  **Loyihani ko'chirib olish**:

    ```bash
    git clone https://github.com/sizning_repo/shop_rocket_api.git
    cd shop_rocket_api
    ```

3.  **Ishga tushirish**:

    ```bash
    cargo run
    ```

    _Dastur birinchi marta ishga tushganda avtomatik ravishda `data.db` faylini yaratadi va barcha migratsiyalarni bajaradi._

4.  **Swagger Hujjatlari**:
    - Brauzerni oching: [http://localhost:8000/swagger-ui/](http://localhost:8000/swagger-ui/)

## 🔑 Foydalanish (API Endpoints)

### Autentifikatsiya (Auth)

- **Ro'yxatdan o'tish**: `POST /auth/register`
  - Body: `{ "email": "ali@mail.uz", "fullname": "Ali Valiyev", "password": "parol" }`
- **Kirish**: `POST /auth/login`
  - Body: `{ "email": "ali@mail.uz", "password": "parol" }`
  - Javob: `{ "token": "...", "email": "...", "fullname": "..." }`

### Mahsulotlar (Products)

- **Ro'yxatni olish**: `GET /products` (Ochiq endpoint)
- **Bitta mahsulotni olish**: `GET /products/{id}` (Ochiq)
- **Yaratish**: `POST /products` (🔒 Token talab qilinadi)
- **O'chirish**: `DELETE /products/{id}` (🔒 Token talab qilinadi)

### 🔒 Himoyalangan Endpointlardan foydalanish

1.  `/auth/login` orqali **token** oling.
2.  Swagger tepasidagi **"Authorize"** tugmasini bosing.
3.  Tokenni kiriting (shunchaki o'zini, `Bearer` so'zi shart emas).
4.  Endi `POST` va `DELETE` metodlarini ishlata olasiz.

## 📝 Muallif

**Shehroz Raxmatov** tomonidan yaratildi.
