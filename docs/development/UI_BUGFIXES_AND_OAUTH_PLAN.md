# 🐛 UI Bug Fixes & OAuth2 Integration Plan

> **⚠️ Архів / не канон (2026-05-18).** Admin modals/OAuth — **FM-012** ✅, **FM-019** baseline ✅. Верифікація: [`ADMIN_A11Y_RUNBOOK.md`](./ADMIN_A11Y_RUNBOOK.md), [`FUNCTION_MANAGEMENT.md`](../catalog/FUNCTION_MANAGEMENT.md) §5.4. Чеклисти нижче — історичні.

## Comprehensive Fixes Based on Quick Click Testing - 2026-01-16

---

## 🎯 Виявлені проблеми (Quick Click Test)

### 1. ⚠️ Забагани сторінок UI відносно авторизації
**Проблеми:**
- Авторизаційні бокси некоректно відображаються
- Проблеми з авторизаційними модалками
- Неправильна обробка стану авторизації на деяких сторінках

**Виправлення:**
- [ ] Перевірити всі модалки авторизації (`/ui/auth`, `/ui/login`)
- [ ] Виправити CSS для авторизаційних блоків (z-index, positioning)
- [ ] Перевірити `requireAuth()` функцію на всіх сторінках
- [ ] Виправити відображення авторизаційних повідомлень
- [ ] Перевірити роботу `updateUI()` після логіну/логауту

### 2. ⚠️ Модалки та кнопки не працюють
**Проблеми:**
- При натисканні кнопки випадає вікно (модалка), але деякі кнопки всередині модалки не працюють
- Проблеми з `onclick` handlers
- Модалки не закриваються коректно
- Focus trap в модалках не працює

**Виправлення:**
- [ ] Перевірити всі модалки в адмін панелі:
  - [ ] Create User Modal (`createUserModal`)
  - [ ] Edit User Modal (`editUserModal`)
  - [ ] Create Worker Modal (`createWorkerModal`)
  - [ ] Create OAuth2 Provider Modal (`createOAuth2Modal`)
  - [ ] Edit OAuth2 Provider Modal (`editOAuth2Modal`)
  - [ ] Create SAML Provider Modal (`createSamlModal`)
  - [ ] Edit SAML Provider Modal (`editSamlModal`)
  - [ ] Create Security Policy Modal (`createPolicyModal`)
  - [ ] Edit Security Policy Modal (`editPolicyModal`)
  - [ ] Create VM Modal (`createVmModal`)
  - [ ] Create Artifact Modal (`createArtifactModal`)
  - [ ] Install Library Modal (`installLibraryModal`)
- [ ] Перевірити `showModal()` та `hideModal()` функції
- [ ] Виправити focus trap (`trapModalFocus`)
- [ ] Перевірити event listeners для всіх кнопок в модалках
- [ ] Виправити закриття модалок при натисканні на backdrop
- [ ] Перевірити обробку Esc для закриття модалок

### 3. 🔐 OAuth2 Авторизація (GitHub, Google, Telegram)
**Завдання:**
- [ ] Додати OAuth2 авторизацію через GitHub
- [ ] Додати OAuth2 авторизацію через Google
- [ ] Додати OAuth2 авторизацію через Telegram (Telegram Login Widget)
- [ ] Оновити сторінку логіну з кнопками OAuth2 провайдерів
- [ ] Додати callback endpoints для OAuth2 провайдерів
- [ ] Реалізувати token exchange для OAuth2 провайдерів
- [ ] Додати user mapping для OAuth2 користувачів
- [ ] Додати конфігурацію OAuth2 провайдерів через адмін панель

### 4. 🛠️ Доробка функцій адмін панелі
**Завдання:**
- [ ] Перевірити всі CRUD операції в адмін панелі:
  - [ ] Users (Create, Edit, Delete) ✅ Частково працює
  - [ ] Workers (Create, Edit, Delete) ✅ Частково працює
  - [ ] VM Instances (Create, Start, Stop, Restart, Delete) ✅ Частково працює
  - [ ] Artifacts (Create, Delete) ✅ Частково працює
  - [ ] Libraries (Install, Delete) ✅ Частково працює
  - [ ] OAuth2 Providers (Create, Edit, Delete) ⏳ Потрібна перевірка
  - [ ] SAML Providers (Create, Edit, Delete) ⏳ Потрібна перевірка
  - [ ] Security Policies (Create, Edit, Delete) ⏳ Потрібна перевірка
  - [ ] Tenants (Create, Edit, Delete) ⏳ Потрібна перевірка
- [ ] Перевірити відображення даних (tables, lists)
- [ ] Виправити помилки валідації форм
- [ ] Додати обробку помилок для всіх операцій
- [ ] Покращити user feedback (notifications, loading states)

---

## 📋 Детальний план виправлень

### Phase 1: Виправлення модалок та кнопок (2-3 дні) ⭐⭐⭐

**День 1: Аудит та виправлення модалок**
- [ ] Створити список всіх модалок в проекті
- [ ] Перевірити кожну модалку на наступні проблеми:
  - `showModal()` та `hideModal()` функції
  - Focus trap (`trapModalFocus`)
  - Закриття при натисканні на backdrop
  - Закриття при натисканні Esc
  - Event listeners для кнопок всередині модалок
  - Form submission в модалках
- [ ] Виправити виявлені проблеми

**День 2: Виправлення авторизаційних боксів**
- [ ] Перевірити `/ui/auth` та `/ui/login` сторінки
- [ ] Виправити CSS для авторизаційних блоків (z-index, positioning, display)
- [ ] Перевірити `requireAuth()` на всіх сторінках
- [ ] Виправити `updateUI()` функцію
- [ ] Перевірити перенаправлення після логіну/логауту

**День 3: Тестування та перевірка**
- [ ] Quick click тест всіх модалок
- [ ] Перевірка авторизації на всіх сторінках
- [ ] Тестування focus trap та keyboard navigation
- [ ] Перевірка закриття модалок різними способами

### Phase 2: OAuth2 Інтеграція (5-7 днів) ⭐⭐⭐

**День 1-2: GitHub OAuth2**
- [ ] Додати GitHub OAuth2 конфігурацію в `SecurityManager`
- [ ] Створити GitHub OAuth2 endpoints (`/api/enterprise/auth/github`, `/api/enterprise/auth/github/callback`)
- [ ] Реалізувати GitHub token exchange
- [ ] Додати user mapping для GitHub користувачів
- [ ] Оновити сторінку логіну з GitHub кнопкою
- [ ] Тестування GitHub OAuth2 flow

**День 3-4: Google OAuth2**
- [ ] Додати Google OAuth2 конфігурацію в `SecurityManager`
- [ ] Створити Google OAuth2 endpoints (`/api/enterprise/auth/google`, `/api/enterprise/auth/google/callback`)
- [ ] Реалізувати Google token exchange (OpenID Connect)
- [ ] Додати user mapping для Google користувачів
- [ ] Оновити сторінку логіну з Google кнопкою
- [ ] Тестування Google OAuth2 flow

**День 5-6: Telegram OAuth2**
- [ ] Додати Telegram Login Widget інтеграцію
- [ ] Створити Telegram OAuth2 endpoints (`/api/enterprise/auth/telegram`, `/api/enterprise/auth/telegram/callback`)
- [ ] Реалізувати Telegram token exchange (Telegram Bot API)
- [ ] Додати user mapping для Telegram користувачів
- [ ] Оновити сторінку логіну з Telegram кнопкою
- [ ] Тестування Telegram OAuth2 flow

**День 7: Консолідація та тестування**
- [ ] Об'єднати всі OAuth2 провайдери в єдиний flow
- [ ] Додати конфігурацію OAuth2 провайдерів через адмін панель
- [ ] Перевірити безпеку OAuth2 flows (state parameter, PKCE)
- [ ] Тестування всіх OAuth2 провайдерів
- [ ] Документація OAuth2 інтеграції

### Phase 3: Доробка адмін панелі (3-5 днів) ⭐⭐

**День 1-2: Перевірка CRUD операцій**
- [ ] Перевірити всі Create операції
- [ ] Перевірити всі Edit операції
- [ ] Перевірити всі Delete операції
- [ ] Виправити помилки в API endpoints
- [ ] Додати валідацію для всіх форм

**День 3-4: Покращення відображення даних**
- [ ] Перевірити відображення таблиць (pagination, sorting, filtering)
- [ ] Виправити відображення списків
- [ ] Додати skeleton loaders для всіх сторінок
- [ ] Покращити error handling та user feedback

**День 5: Тестування та документація**
- [ ] E2E тести для всіх CRUD операцій
- [ ] Перевірка всіх сторінок адмін панелі
- [ ] Оновлення документації адмін панелі

---

## 🚀 Початок роботи

### Негайні кроки (сьогодні):

1. **Виправити модалки та кнопки** (Priority 1)
   - Перевірити `showModal()` та `hideModal()` функції
   - Виправити focus trap
   - Перевірити всі `onclick` handlers в модалках

2. **Виправити авторизаційні бокси** (Priority 1)
   - Перевірити `/ui/auth` та `/ui/login` сторінки
   - Виправити CSS для авторизаційних блоків

3. **Почти OAuth2 інтеграцію** (Priority 2)
   - Додати GitHub OAuth2 (найпростіший)
   - Потім Google OAuth2
   - Потім Telegram OAuth2

4. **Доробити адмін панель** (Priority 2)
   - Перевірити всі CRUD операції
   - Виправити помилки валідації

---

## 📊 Прогрес

**Phase 1: Модалки та кнопки** - 0% ⏳
- Аудит модалок: 0% ⏳
- Виправлення авторизаційних боксів: 0% ⏳
- Тестування: 0% ⏳

**Phase 2: OAuth2 Інтеграція** - 0% ⏳
- GitHub OAuth2: 0% ⏳
- Google OAuth2: 0% ⏳
- Telegram OAuth2: 0% ⏳

**Phase 3: Доробка адмін панелі** - 0% ⏳
- CRUD операції: 0% ⏳
- Відображення даних: 0% ⏳
- Тестування: 0% ⏳

---

**Загальний прогрес: 0%** ⏳

**Очікуваний час виконання: 10-15 днів**

**Last Updated**: 2026-01-16
