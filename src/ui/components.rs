//! UI Components - Компоненты пользовательского интерфейса
//! 
//! Этот модуль предоставляет:
//! - UI компоненты
//! - Визуализацию
//! - Интерактивность
//! - Отзывчивость

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Базовый компонент UI
pub trait UiComponent {
    fn render(&self) -> String;
    fn update(&mut self, data: &str) -> Result<(), String>;
    fn get_id(&self) -> &str;
}

/// Карточка метрики
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCard {
    pub id: String,
    pub title: String,
    pub value: String,
    pub unit: Option<String>,
    pub trend: Option<Trend>,
    pub color: String,
    pub icon: Option<String>,
}

impl UiComponent for MetricCard {
    fn render(&self) -> String {
        let trend_html = if let Some(trend) = &self.trend {
            format!(
                r#"<div class="trend trend-{}">
                    <span class="trend-icon">{}</span>
                    <span class="trend-value">{}</span>
                </div>"#,
                trend.direction.to_string().to_lowercase(),
                trend.icon,
                trend.value
            )
        } else {
            String::new()
        };

        let icon_html = if let Some(icon) = &self.icon {
            format!(r#"<div class="card-icon">{}</div>"#, icon)
        } else {
            String::new()
        };

        format!(
            r#"<div class="metric-card" id="{}">
                <div class="card-header">
                    <h3>{}</h3>
                    {}
                </div>
                <div class="card-body">
                    <div class="metric-value">
                        {}{}
                    </div>
                    {}
                </div>
            </div>"#,
            self.id,
            self.title,
            icon_html,
            self.value,
            self.unit.as_ref().map(|u| format!(" {}", u)).unwrap_or_default(),
            trend_html
        )
    }

    fn update(&mut self, data: &str) -> Result<(), String> {
        // Парсим JSON данные для обновления
        let data: serde_json::Value = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse data: {}", e))?;
        
        if let Some(value) = data.get("value").and_then(|v| v.as_str()) {
            self.value = value.to_string();
        }
        
        if let Some(trend_data) = data.get("trend") {
            self.trend = serde_json::from_value(trend_data.clone()).ok();
        }
        
        Ok(())
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// График
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub id: String,
    pub title: String,
    pub chart_type: ChartType,
    pub data: ChartData,
    pub options: ChartOptions,
}

impl UiComponent for Chart {
    fn render(&self) -> String {
        let chart_data = serde_json::to_string(&self.data)
            .unwrap_or_else(|_| "{}".to_string());
        
        let chart_options = serde_json::to_string(&self.options)
            .unwrap_or_else(|_| "{}".to_string());

        format!(
            r#"<div class="chart-container" id="{}">
                <h3>{}</h3>
                <canvas id="chart-{}"></canvas>
                <script>
                    createChart('chart-{}', '{}', {}, {});
                </script>
            </div>"#,
            self.id,
            self.title,
            self.id,
            self.id,
            self.chart_type.to_string(),
            chart_data,
            chart_options
        )
    }

    fn update(&mut self, data: &str) -> Result<(), String> {
        let new_data: ChartData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse chart data: {}", e))?;
        
        self.data = new_data;
        Ok(())
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Таблица
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    pub title: String,
    pub columns: Vec<TableColumn>,
    pub data: Vec<HashMap<String, String>>,
    pub pagination: Option<Pagination>,
    pub sorting: Option<Sorting>,
}

impl UiComponent for Table {
    fn render(&self) -> String {
        let headers = self.columns.iter()
            .map(|col| format!(r#"<th data-sort="{}">{}</th>"#, col.key, col.title))
            .collect::<Vec<_>>()
            .join("");

        let rows = self.data.iter()
            .map(|row| {
                let cells = self.columns.iter()
                    .map(|col| {
                        let value = row.get(&col.key).unwrap_or(&String::new());
                        format!(r#"<td>{}</td>"#, value)
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!(r#"<tr>{}</tr>"#, cells)
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div class="table-container" id="{}">
                <h3>{}</h3>
                <table class="data-table">
                    <thead>
                        <tr>{}</tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>"#,
            self.id,
            self.title,
            headers,
            rows
        )
    }

    fn update(&mut self, data: &str) -> Result<(), String> {
        let new_data: Vec<HashMap<String, String>> = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse table data: {}", e))?;
        
        self.data = new_data;
        Ok(())
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Форма
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub title: String,
    pub fields: Vec<FormField>,
    pub submit_url: String,
    pub method: String,
}

impl UiComponent for Form {
    fn render(&self) -> String {
        let fields = self.fields.iter()
            .map(|field| field.render())
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<div class="form-container" id="{}">
                <h3>{}</h3>
                <form method="{}" action="{}">
                    {}
                    <div class="form-actions">
                        <button type="submit" class="btn-primary">Submit</button>
                        <button type="reset" class="btn-secondary">Reset</button>
                    </div>
                </form>
            </div>"#,
            self.id,
            self.title,
            self.method,
            self.submit_url,
            fields
        )
    }

    fn update(&mut self, data: &str) -> Result<(), String> {
        let form_data: HashMap<String, String> = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse form data: {}", e))?;
        
        for field in &mut self.fields {
            if let Some(value) = form_data.get(&field.name) {
                field.value = value.clone();
            }
        }
        
        Ok(())
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

/// Модальное окно
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modal {
    pub id: String,
    pub title: String,
    pub content: String,
    pub size: ModalSize,
    pub show_close: bool,
    pub backdrop: bool,
}

impl UiComponent for Modal {
    fn render(&self) -> String {
        let backdrop_class = if self.backdrop { "modal-backdrop" } else { "" };
        let size_class = self.size.to_string().to_lowercase();

        format!(
            r#"<div class="modal {}" id="{}">
                <div class="modal-dialog modal-{}">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h3>{}</h3>
                            {}
                        </div>
                        <div class="modal-body">
                            {}
                        </div>
                    </div>
                </div>
            </div>"#,
            backdrop_class,
            self.id,
            size_class,
            self.title,
            if self.show_close {
                r#"<button type="button" class="modal-close" onclick="closeModal('{}')">&times;</button>"#.to_string()
            } else {
                String::new()
            },
            self.content
        )
    }

    fn update(&mut self, data: &str) -> Result<(), String> {
        let modal_data: HashMap<String, String> = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse modal data: {}", e))?;
        
        if let Some(content) = modal_data.get("content") {
            self.content = content.clone();
        }
        
        if let Some(title) = modal_data.get("title") {
            self.title = title.clone();
        }
        
        Ok(())
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

// Вспомогательные структуры

/// Тренд метрики
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub direction: TrendDirection,
    pub value: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

impl std::fmt::Display for TrendDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrendDirection::Up => write!(f, "up"),
            TrendDirection::Down => write!(f, "down"),
            TrendDirection::Stable => write!(f, "stable"),
        }
    }
}

/// Тип графика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Doughnut,
    Area,
}

impl std::fmt::Display for ChartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChartType::Line => write!(f, "line"),
            ChartType::Bar => write!(f, "bar"),
            ChartType::Pie => write!(f, "pie"),
            ChartType::Doughnut => write!(f, "doughnut"),
            ChartType::Area => write!(f, "area"),
        }
    }
}

/// Данные графика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub data: Vec<f64>,
    pub border_color: String,
    pub background_color: String,
}

/// Опции графика
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartOptions {
    pub responsive: bool,
    pub maintain_aspect_ratio: bool,
    pub scales: Option<Scales>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scales {
    pub y: Option<Scale>,
    pub x: Option<Scale>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    pub begin_at_zero: bool,
    pub max: Option<f64>,
    pub min: Option<f64>,
}

/// Колонка таблицы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub key: String,
    pub title: String,
    pub sortable: bool,
    pub width: Option<String>,
}

/// Пагинация
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub current_page: u32,
    pub total_pages: u32,
    pub page_size: u32,
    pub total_items: u32,
}

/// Сортировка
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sorting {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Поле формы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub value: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub options: Option<Vec<String>>,
}

impl FormField {
    fn render(&self) -> String {
        let required_attr = if self.required { "required" } else { "" };
        let placeholder_attr = self.placeholder.as_ref()
            .map(|p| format!("placeholder=\"{}\"", p))
            .unwrap_or_default();

        match &self.field_type {
            FieldType::Text => {
                format!(
                    r#"<div class="form-group">
                        <label for="{}">{}</label>
                        <input type="text" name="{}" value="{}" {} {} />
                    </div>"#,
                    self.name, self.label, self.name, self.value, required_attr, placeholder_attr
                )
            }
            FieldType::Number => {
                format!(
                    r#"<div class="form-group">
                        <label for="{}">{}</label>
                        <input type="number" name="{}" value="{}" {} {} />
                    </div>"#,
                    self.name, self.label, self.name, self.value, required_attr, placeholder_attr
                )
            }
            FieldType::Select => {
                let options = self.options.as_ref()
                    .map(|opts| {
                        opts.iter()
                            .map(|opt| format!(r#"<option value="{}">{}</option>"#, opt, opt))
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .unwrap_or_default();

                format!(
                    r#"<div class="form-group">
                        <label for="{}">{}</label>
                        <select name="{}" {}>
                            {}
                        </select>
                    </div>"#,
                    self.name, self.label, self.name, required_attr, options
                )
            }
            FieldType::Textarea => {
                format!(
                    r#"<div class="form-group">
                        <label for="{}">{}</label>
                        <textarea name="{}" {} {}>{}</textarea>
                    </div>"#,
                    self.name, self.label, self.name, required_attr, placeholder_attr, self.value
                )
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Select,
    Textarea,
}

/// Размер модального окна
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl std::fmt::Display for ModalSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModalSize::Small => write!(f, "sm"),
            ModalSize::Medium => write!(f, "md"),
            ModalSize::Large => write!(f, "lg"),
            ModalSize::ExtraLarge => write!(f, "xl"),
        }
    }
}

/// Менеджер компонентов
pub struct ComponentManager {
    components: HashMap<String, Box<dyn UiComponent>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn UiComponent>) {
        let id = component.get_id().to_string();
        self.components.insert(id, component);
    }

    pub fn get_component(&self, id: &str) -> Option<&Box<dyn UiComponent>> {
        self.components.get(id)
    }

    pub fn update_component(&mut self, id: &str, data: &str) -> Result<(), String> {
        if let Some(component) = self.components.get_mut(id) {
            component.update(data)
        } else {
            Err(format!("Component with id '{}' not found", id))
        }
    }

    pub fn render_all(&self) -> String {
        self.components.values()
            .map(|component| component.render())
            .collect::<Vec<_>>()
            .join("\n")
    }
} 