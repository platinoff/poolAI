# 🧪 TLS Testing Plan - PoolAI
## План тестування TLS/HTTPS функціональності

**Дата створення**: 2026-01-09  
**Версія**: 1.0  
**Статус**: План готовий до виконання

---

## 📋 Тестування (Етап 1.3)

### 1.3.1 Тестування з self-signed сертифікатами

**Мета**: Перевірити роботу HTTPS з self-signed сертифікатами для розробки

**Кроки**:
1. Створити self-signed сертифікат:
   ```bash
   openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem -days 365 -nodes
   ```

2. Запустити сервер з feature `https`:
   ```bash
   cargo run --release --features https
   ```

3. Перевірити HTTPS з'єднання:
   ```bash
   curl -k https://localhost:8443/api/v1/status
   ```

4. Перевірити HSTS headers:
   ```bash
   curl -k -I https://localhost:8443/api/v1/status | grep -i strict-transport
   ```

**Очікуваний результат**:
- ✅ Сервер запускається з HTTPS
- ✅ API endpoints доступні через HTTPS
- ✅ HSTS headers додаються автоматично
- ✅ Fallback до HTTP якщо сертифікати не знайдено

---

### 1.3.2 Тестування з Let's Encrypt сертифікатами

**Мета**: Перевірити роботу HTTPS з production сертифікатами

**Кроки**:
1. Отримати Let's Encrypt сертифікат (через certbot):
   ```bash
   certbot certonly --standalone -d poolai.example.com
   ```

2. Налаштувати конфігурацію:
   ```toml
   [https]
   enabled = true
   cert_path = "/etc/letsencrypt/live/poolai.example.com/fullchain.pem"
   key_path = "/etc/letsencrypt/live/poolai.example.com/privkey.pem"
   ```

3. Запустити сервер:
   ```bash
   cargo run --release --features https
   ```

4. Перевірити HTTPS з'єднання:
   ```bash
   curl https://poolai.example.com:8443/api/v1/status
   ```

**Очікуваний результат**:
- ✅ Сервер запускається з HTTPS
- ✅ Сертифікат валідний (без -k flag)
- ✅ HSTS headers додаються
- ✅ Certificate Transparency (коли буде реалізовано)

---

### 1.3.3 Тестування HSTS headers

**Мета**: Перевірити правильність HSTS headers

**Тести**:
1. Перевірити наявність HSTS header:
   ```bash
   curl -k -I https://localhost:8443/api/v1/status
   ```

2. Перевірити значення HSTS header:
   - Має містити `max-age=31536000`
   - Має містити `includeSubDomains` (якщо увімкнено)

3. Перевірити відсутність HSTS на HTTP:
   ```bash
   curl -I http://localhost:8080/api/v1/status
   ```

**Очікуваний результат**:
- ✅ HSTS header присутній на HTTPS responses
- ✅ HSTS header відсутній на HTTP responses
- ✅ Правильне значення max-age
- ✅ Правильне значення includeSubDomains

---

### 1.3.4 Тестування TLS версій

**Мета**: Перевірити підтримку TLS версій

**Тести**:
1. Перевірити підтримку TLS 1.3:
   ```bash
   openssl s_client -connect localhost:8443 -tls1_3
   ```

2. Перевірити відсутність підтримки TLS 1.2 та старіших:
   ```bash
   openssl s_client -connect localhost:8443 -tls1_2
   ```

**Очікуваний результат**:
- ✅ TLS 1.3 підтримується
- ✅ TLS 1.2 та старіші не підтримуються (якщо налаштовано)

---

### 1.3.5 Тестування cipher suites

**Мета**: Перевірити використання правильних cipher suites

**Тести**:
1. Перевірити доступні cipher suites:
   ```bash
   openssl s_client -connect localhost:8443 -cipher 'TLS_AES_256_GCM_SHA384'
   ```

2. Перевірити пріоритет cipher suites:
   - `TLS_AES_256_GCM_SHA384` (пріоритет)
   - `TLS_CHACHA20_POLY1305_SHA256`
   - `TLS_AES_128_GCM_SHA256`

**Очікуваний результат**:
- ✅ Тільки безпечні cipher suites доступні
- ✅ Правильний пріоритет cipher suites

---

### 1.3.6 Тестування fallback до HTTP

**Мета**: Перевірити fallback до HTTP якщо сертифікати не знайдено

**Тести**:
1. Запустити сервер без сертифікатів:
   ```bash
   cargo run --release --features https
   # (без cert.pem та key.pem)
   ```

2. Перевірити що сервер запускається на HTTP:
   ```bash
   curl http://localhost:8080/api/v1/status
   ```

**Очікуваний результат**:
- ✅ Сервер запускається на HTTP якщо сертифікати не знайдено
- ✅ Логується warning про відсутність сертифікатів
- ✅ API endpoints доступні через HTTP

---

## 📊 Чеклист тестування

### Self-signed сертифікати
- [ ] Створення self-signed сертифікату
- [ ] Запуск сервера з HTTPS
- [ ] Перевірка HTTPS з'єднання
- [ ] Перевірка HSTS headers
- [ ] Перевірка API endpoints через HTTPS

### Let's Encrypt сертифікати
- [ ] Отримання Let's Encrypt сертифікату
- [ ] Налаштування конфігурації
- [ ] Запуск сервера з production сертифікатами
- [ ] Перевірка валідності сертифікату
- [ ] Перевірка автоматичного оновлення сертифікатів

### HSTS headers
- [ ] Наявність HSTS header на HTTPS
- [ ] Відсутність HSTS header на HTTP
- [ ] Правильне значення max-age
- [ ] Правильне значення includeSubDomains

### TLS версії
- [ ] Підтримка TLS 1.3
- [ ] Відсутність підтримки TLS 1.2 та старіших

### Cipher suites
- [ ] Доступність безпечних cipher suites
- [ ] Правильний пріоритет cipher suites

### Fallback до HTTP
- [ ] Fallback до HTTP якщо сертифікати не знайдено
- [ ] Логування warning про відсутність сертифікатів

---

## 🎯 Наступні кроки після тестування

1. **Документування результатів**:
   - Створити звіт про тестування
   - Документувати виявлені проблеми
   - Документувати рекомендації

2. **Виправлення проблем**:
   - Виправити виявлені проблеми
   - Додати додаткові тести
   - Оновити документацію

3. **Підготовка до production**:
   - Налаштувати автоматичне оновлення сертифікатів
   - Додати моніторинг TLS з'єднань
   - Додати алерти для проблем з сертифікатами

---

**Підготовлено**: Rust Architect  
**Дата**: 2026-01-09  
**Версія**: 1.0 - TLS Testing Plan
