# 📚 Libs Module - Документація

**Модуль**: Library Management  
**Статус**: 🚧 В розробці (40% готово)  
**Stage**: 3

---

## 🎯 Призначення

Libs Module забезпечує управління бібліотеками моделей (libtorch, тощо) з підтримкою:
- Автоматичного завантаження та встановлення
- Версіонування бібліотек
- Управління залежностями
- API для управління через REST

---

## 📋 API Endpoints

### Список бібліотек
```http
GET /api/v1/libraries
```

**Відповідь:**
```json
[
  {
    "name": "libtorch",
    "version": "2.1.0",
    "path": "/path/to/libtorch",
    "dependencies": [],
    "metadata": {
      "installed_at": "2025-12-05T10:00:00Z"
    }
  }
]
```

### Інформація про бібліотеку
```http
GET /api/v1/libraries/:name
```

### Встановлення бібліотеки
```http
POST /api/v1/libraries/:name/install
Content-Type: application/json

{
  "version": "2.1.0"
}
```

### Видалення бібліотеки
```http
POST /api/v1/libraries/:name/uninstall
```

### Оновлення бібліотеки
```http
POST /api/v1/libraries/:name/update
```

---

## 🏗️ Архітектура

### Компоненти

1. **LibraryManager** - Головний інтерфейс
2. **LibraryRegistry** - Реєстр бібліотек
3. **VersionManager** - Управління версіями
4. **DependencyResolver** - Резолюція залежностей

### Thread Safety

- `Arc<RwLock<LibraryManager>>` для shared state
- `OnceLock` для глобальної ініціалізації
- Async/await для всіх I/O операцій

---

## 📝 Приклад використання

```rust
use poolai::libs::{get_global_manager, LibraryType};

// Отримати менеджер
if let Some(manager) = get_global_manager() {
    let manager = manager.read().await;
    
    // Встановити бібліотеку
    let lib = manager.install_library("libtorch", "2.1.0", LibraryType::ModelLibrary).await?;
    
    // Отримати інформацію
    let info = manager.get_library("libtorch").await;
    
    // Список бібліотек
    let libraries = manager.list_libraries().await;
}
```

---

## 🔄 Наступні кроки

1. Реалізувати завантаження бібліотек
2. Покращити dependency resolution
3. Додати semantic versioning
4. Додати тести
5. Документація Rustdoc

---

**Модуль готовий до базового використання!** 🚀

