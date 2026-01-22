# Pruning Strategies (ML.1)
## Дата: 2026-01-22

Модуль стратегій pruning для оптимізації моделей шляхом видалення менш важливих ваг.

---

## 🎯 Мета

Pruning strategies для зменшення розміру моделі та покращення продуктивності:
- Magnitude-based pruning
- Structured pruning
- Unstructured pruning
- Iterative pruning
- Pruning evaluation

---

## 📚 API

### Pruning Configuration

```rust
use poolai::ml::optimization::{PruningConfig, PruningStrategy};

let mut config = PruningConfig::default_config();
config.strategy = PruningStrategy::MagnitudeBased;
config.ratio = 0.2; // Prune 20%
```

### Apply Pruning

```rust
use poolai::ml::optimization::{apply_pruning, PruningConfig, PruningStrategy};

let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
let mut config = PruningConfig::default_config();
config.strategy = PruningStrategy::MagnitudeBased;
config.ratio = 0.25;

let result = apply_pruning(&weights, &config);
println!("Pruned: {} weights, Compression: {:.2}x",
    result.pruned_count, result.compression_ratio);
```

### Iterative Pruning

```rust
use poolai::ml::optimization::{apply_iterative_pruning, PruningConfig};

let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05];
let mut config = PruningConfig::default_config();
config.iterative = true;
config.iterations = 3;
config.ratio = 0.3; // Total 30%, 10% per iteration

let result = apply_iterative_pruning(&weights, &config);
```

### Evaluate Pruning

```rust
use poolai::ml::optimization::evaluate_pruning;

let before = vec![1.0, 2.0, 3.0, 4.0];
let after = vec![1.0, 0.0, 3.0, 0.0];

let result = evaluate_pruning(&before, &after);
println!("Pruned: {} weights", result.pruned_count);
```

---

## 📊 Структури

### PruningConfig

- `strategy: PruningStrategy` - Стратегія pruning
- `ratio: f32` - Відсоток ваг для pruning (0.0-1.0)
- `iterative: bool` - Використовувати ітеративний pruning
- `iterations: u32` - Кількість ітерацій

### PruningResult

- `strategy: PruningStrategy` - Використана стратегія
- `weights_before: usize` - Кількість ваг до pruning
- `weights_after: usize` - Кількість ваг після pruning
- `pruned_count: usize` - Кількість видалених ваг
- `compression_ratio: f64` - Коефіцієнт стиснення
- `accuracy_drop: f64` - Оцінка падіння точності

### PruningStrategy

- `MagnitudeBased` - Pruning на основі абсолютного значення (за замовчуванням)
- `Structured` - Pruning цілих каналів/фільтрів
- `Unstructured` - Fine-grained pruning окремих ваг

---

## 🔍 Функціональність

### Magnitude-Based Pruning

Видаляє ваги з найменшими абсолютними значеннями:

1. Сортує ваги за абсолютним значенням
2. Залишає топ-N ваг (найбільші за модулем)
3. Встановлює інші ваги в 0

**Переваги**:
- Простий та ефективний
- Добре працює для більшості моделей
- Мінімальна втрата точності

### Structured Pruning

Видаляє цілі канали або фільтри:

1. Групує ваги в канали
2. Обчислює magnitude кожного каналу
3. Видаляє канали з найменшим magnitude

**Переваги**:
- Краща продуктивність на GPU
- Менше операцій під час inference
- Підтримка hardware acceleration

### Unstructured Pruning

Fine-grained pruning окремих ваг:

1. Аналізує кожну вагу окремо
2. Видаляє найменш важливі ваги
3. Максимальна гнучкість

**Переваги**:
- Максимальне стиснення
- Точний контроль над pruning
- Може працювати з будь-якою архітектурою

### Iterative Pruning

Поступовий pruning з кількома ітераціями:

1. Розділяє загальний ratio на кілька ітерацій
2. Застосовує pruning поступово
3. Може включати retraining між ітераціями

**Переваги**:
- Менша втрата точності
- Більш стабільний процес
- Можливість адаптації

---

## 🧪 Тестування

```bash
cargo test --test ml_pruning_integration --features ml
```

Тести покривають:
- Всі стратегії pruning
- Ітеративний pruning
- Оцінку pruning
- Edge cases (empty, zero ratio, full ratio)
- Великі моделі

---

## 📝 Приклади Використання

### Базовий Pruning

```rust
use poolai::ml::optimization::{apply_pruning, PruningConfig, PruningStrategy};

fn main() {
    let weights = vec![1.0, 0.5, 0.1, 2.0, 0.3, 0.05, 3.0, 0.2];
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::MagnitudeBased;
    config.ratio = 0.25; // Prune 25%

    let result = apply_pruning(&weights, &config);
    println!("Pruned: {} weights", result.pruned_count);
    println!("Compression: {:.2}x", result.compression_ratio);
    println!("Estimated accuracy drop: {:.2}%", result.accuracy_drop * 100.0);
}
```

### Structured Pruning

```rust
use poolai::ml::optimization::{apply_pruning, PruningConfig, PruningStrategy};

fn main() {
    let weights: Vec<f64> = (0..1000).map(|i| (i % 10) as f64).collect();
    let mut config = PruningConfig::default_config();
    config.strategy = PruningStrategy::Structured;
    config.ratio = 0.3; // Prune 30% of channels

    let result = apply_pruning(&weights, &config);
    println!("Structured pruning: {} weights pruned", result.pruned_count);
}
```

### Iterative Pruning

```rust
use poolai::ml::optimization::{apply_iterative_pruning, PruningConfig};

fn main() {
    let weights: Vec<f64> = (0..10000).map(|i| (i % 100) as f64).collect();
    let mut config = PruningConfig::default_config();
    config.iterative = true;
    config.iterations = 5;
    config.ratio = 0.5; // Total 50%, 10% per iteration

    let result = apply_iterative_pruning(&weights, &config);
    println!("Iterative pruning: {} weights pruned over {} iterations",
        result.pruned_count, config.iterations);
}
```

### Evaluate Pruning Impact

```rust
use poolai::ml::optimization::{apply_pruning, evaluate_pruning, PruningConfig};

fn main() {
    let original_weights = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    
    // Apply pruning
    let mut config = PruningConfig::default_config();
    config.ratio = 0.25;
    let result = apply_pruning(&original_weights, &config);
    
    // In real scenario, would get pruned weights from model
    // For demo, simulate pruned weights
    let pruned_weights = vec![1.0, 0.0, 3.0, 0.0, 5.0, 0.0, 7.0, 0.0];
    
    // Evaluate impact
    let evaluation = evaluate_pruning(&original_weights, &pruned_weights);
    println!("Evaluation: {} weights pruned", evaluation.pruned_count);
    println!("Accuracy drop estimate: {:.2}%", evaluation.accuracy_drop * 100.0);
}
```

---

## 🔗 Інтеграція

### З Optimization Module

Pruning інтегровано з `OptimizationProfile`:
- `pruning_ratio` в профілі
- Комбінація з quantization
- Використання з profiling

### З AutoML Module

Pruning може використовуватися в AutoML pipeline:
- Автоматичний вибір стратегії
- Оптимізація pruning ratio
- Оцінка impact на accuracy

---

## 📈 Майбутні Покращення

- [ ] Реальна реалізація retraining після pruning
- [ ] Adaptive pruning ratio
- [ ] Layer-wise pruning strategies
- [ ] Pruning з урахуванням accuracy
- [ ] Hardware-aware pruning
- [ ] Pruning для специфічних архітектур (CNN, RNN, Transformer)
- [ ] Pruning з differential privacy
- [ ] Automated pruning pipeline

---

**Версія**: v0.2.2  
**Дата**: 2026-01-22  
**Статус**: ✅ Реалізовано (Priority 4 - Ітерація 4)
