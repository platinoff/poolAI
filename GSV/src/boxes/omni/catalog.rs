//! OmniRouter catalog — provider registry + model registry.
//!
//! Source of truth: the "AI providers by opencode" sheet (Aug 2026):
//! recommended list (GPT 5.2, GPT 5.1 Codex, Claude Opus 4.5, Claude Sonnet 4.5,
//! MiniMax M2.1, Gemini 3 Pro) plus provider specs (models.dev / official docs).
//! Token windows / max output are taken from that sheet verbatim; `None` means
//! "varies by model" (the sheet does not publish a number).

/// A provider (API vendor / aggregator).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSpec {
    /// Canonical provider id used in config, routing and `X-Omni-Provider`.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Region: `Global` or `China`.
    pub region: &'static str,
    /// Has a free tier / free models.
    pub free: bool,
    /// Default OpenAI-compatible base URL (`/v1`-style). Empty when the provider
    /// has no public API (must be configured manually via `omni.toml` / env).
    pub default_base_url: &'static str,
    /// One-line note (models / pricing from the sheet).
    pub notes: &'static str,
}

/// A single model entry in the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpec {
    /// Canonical model id (what the caller sends as `model`).
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Owning provider id (`ProviderSpec::id`).
    pub provider: &'static str,
    /// Token window (context), from the sheet.
    pub context_window: Option<u32>,
    /// Max output token capacity, from the sheet.
    pub max_output: Option<u32>,
    /// Free tier / free model.
    pub free: bool,
    /// On the sheet's recommended list.
    pub recommended: bool,
    /// Tier: flagship | code | agent | fast | open | aggregator.
    pub tier: &'static str,
}

/// Catalog of known providers (sheet rows, deduplicated).
pub fn providers() -> &'static [ProviderSpec] {
    &[
        ProviderSpec {
            id: "openai",
            name: "OpenAI",
            region: "Global",
            free: false,
            default_base_url: "https://api.openai.com/v1",
            notes: "GPT-5.2 / GPT-5.2-Codex · 400K ctx / 128K out",
        },
        ProviderSpec {
            id: "anthropic",
            name: "Anthropic",
            region: "Global",
            free: false,
            default_base_url: "https://api.anthropic.com/v1",
            notes: "Claude Opus 4.5 / Sonnet 4.5 (agent) · 200K ctx / 64K out",
        },
        ProviderSpec {
            id: "google",
            name: "Google",
            region: "Global",
            free: false,
            default_base_url: "https://generativelanguage.googleapis.com/v1beta/openai",
            notes: "Gemini 3 Pro · 1M ctx / 65K out",
        },
        ProviderSpec {
            id: "minimax",
            name: "MiniMax",
            region: "China",
            free: false,
            default_base_url: "https://api.minimax.io/v1",
            notes: "M2.1 · 1M ctx / 131K out",
        },
        ProviderSpec {
            id: "deepseek",
            name: "DeepSeek",
            region: "China",
            free: false,
            default_base_url: "https://api.deepseek.com/v1",
            notes: "V4-Pro / V4-Flash · 1M ctx / 384K out (free trial credits)",
        },
        ProviderSpec {
            id: "moonshot",
            name: "Moonshot AI (Kimi)",
            region: "China",
            free: false,
            default_base_url: "https://api.moonshot.cn/v1",
            notes: "Kimi K3 (2.8T) / K2.7 Code · 1M / 256K ctx",
        },
        ProviderSpec {
            id: "zai",
            name: "Z.AI (Zhipu)",
            region: "China",
            free: false,
            default_base_url: "https://open.bigmodel.cn/api/paas/v4",
            notes: "GLM-4.6 · 200K ctx / 128K out (Coding plan has free models)",
        },
        ProviderSpec {
            id: "qwen",
            name: "Alibaba (Qwen)",
            region: "China",
            free: false,
            default_base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1",
            notes: "Qwen3 Coder 480B · 256K ctx (1M w/ YaRN) / 65K out",
        },
        ProviderSpec {
            id: "openrouter",
            name: "OpenRouter",
            region: "Global",
            free: true,
            default_base_url: "https://openrouter.ai/api/v1",
            notes: "All models incl. `:free` variants",
        },
        ProviderSpec {
            id: "groq",
            name: "Groq",
            region: "Global",
            free: true,
            default_base_url: "https://api.groq.com/openai/v1",
            notes: "Llama / Qwen / DeepSeek (fast inference) · free tier",
        },
        ProviderSpec {
            id: "cerebras",
            name: "Cerebras",
            region: "Global",
            free: true,
            default_base_url: "https://api.cerebras.ai/v1",
            notes: "Qwen3 Coder 480B, etc. · free tier",
        },
        ProviderSpec {
            id: "nvidia",
            name: "NVIDIA (build.nvidia.com)",
            region: "Global",
            free: true,
            default_base_url: "https://integrate.api.nvidia.com/v1",
            notes: "Nemotron / open models · free",
        },
        ProviderSpec {
            id: "huggingface",
            name: "Hugging Face",
            region: "Global",
            free: true,
            default_base_url: "https://router.huggingface.co/v1",
            notes: "Inference Providers · Kimi-K2, GLM-4.6, etc. · free",
        },
        ProviderSpec {
            id: "copilot",
            name: "GitHub Copilot",
            region: "Global",
            free: true,
            default_base_url: "",
            notes: "GPT-5.x / Claude / Gemini via Copilot · Free plan (configure base_url)",
        },
        ProviderSpec {
            id: "opencode-zen",
            name: "OpenCode Zen",
            region: "Global",
            free: false,
            default_base_url: "",
            notes: "GPT-5.1 Codex, Claude Sonnet 4.5, Qwen3 Coder · cheap per-token (configure)",
        },
        ProviderSpec {
            id: "opencode-go",
            name: "OpenCode Go",
            region: "Global",
            free: false,
            default_base_url: "",
            notes: "Open coding models · low-cost sub (configure)",
        },
        ProviderSpec {
            id: "302ai",
            name: "302.AI",
            region: "China",
            free: false,
            default_base_url: "https://api.302.ai/v1",
            notes: "Aggregate of Chinese + global models",
        },
    ]
}

/// Full model registry (sheet rows). Multiple entries may share a model `id`
/// (same model hosted by different providers), so lookups return a list.
pub fn models() -> &'static [ModelSpec] {
    &[
        // ── Recommended (sheet row 2) ───────────────────────────────
        ModelSpec {
            id: "gpt-5.2",
            name: "GPT-5.2",
            provider: "openai",
            context_window: Some(400_000),
            max_output: Some(128_000),
            free: false,
            recommended: true,
            tier: "flagship",
        },
        ModelSpec {
            id: "gpt-5.2-codex",
            name: "GPT-5.2 Codex",
            provider: "openai",
            context_window: Some(400_000),
            max_output: Some(128_000),
            free: false,
            recommended: true,
            tier: "code",
        },
        ModelSpec {
            id: "claude-opus-4.5",
            name: "Claude Opus 4.5",
            provider: "anthropic",
            context_window: Some(200_000),
            max_output: Some(64_000),
            free: false,
            recommended: true,
            tier: "flagship",
        },
        ModelSpec {
            id: "claude-sonnet-4.5",
            name: "Claude Sonnet 4.5",
            provider: "anthropic",
            context_window: Some(200_000),
            max_output: Some(64_000),
            free: false,
            recommended: true,
            tier: "agent",
        },
        ModelSpec {
            id: "gemini-3-pro",
            name: "Gemini 3 Pro",
            provider: "google",
            context_window: Some(1_000_000),
            max_output: Some(65_000),
            free: false,
            recommended: true,
            tier: "flagship",
        },
        ModelSpec {
            id: "minimax-m2.1",
            name: "MiniMax M2.1",
            provider: "minimax",
            context_window: Some(1_000_000),
            max_output: Some(131_000),
            free: false,
            recommended: true,
            tier: "flagship",
        },
        // ── Chinese models (sheet table 2) ──────────────────────────
        ModelSpec {
            id: "deepseek-v4-pro",
            name: "DeepSeek V4-Pro",
            provider: "deepseek",
            context_window: Some(1_000_000),
            max_output: Some(384_000),
            free: false,
            recommended: false,
            tier: "flagship",
        },
        ModelSpec {
            id: "deepseek-v4-flash",
            name: "DeepSeek V4-Flash",
            provider: "deepseek",
            context_window: Some(1_000_000),
            max_output: Some(384_000),
            free: false,
            recommended: false,
            tier: "fast",
        },
        ModelSpec {
            id: "kimi-k3",
            name: "Kimi K3",
            provider: "moonshot",
            context_window: Some(1_000_000),
            max_output: Some(128_000),
            free: false,
            recommended: false,
            tier: "flagship",
        },
        ModelSpec {
            id: "kimi-k2.7-code",
            name: "Kimi K2.7 Code",
            provider: "moonshot",
            context_window: Some(256_000),
            max_output: Some(32_000),
            free: false,
            recommended: false,
            tier: "code",
        },
        ModelSpec {
            id: "glm-4.6",
            name: "GLM-4.6",
            provider: "zai",
            context_window: Some(200_000),
            max_output: Some(128_000),
            free: false,
            recommended: false,
            tier: "flagship",
        },
        ModelSpec {
            id: "qwen3-coder-480b",
            name: "Qwen3 Coder 480B",
            provider: "qwen",
            context_window: Some(256_000),
            max_output: Some(65_000),
            free: false,
            recommended: false,
            tier: "code",
        },
        // ── Free / fast hosts (sheet table 1) ───────────────────────
        ModelSpec {
            id: "qwen3-coder-480b",
            name: "Qwen3 Coder 480B",
            provider: "cerebras",
            context_window: Some(256_000),
            max_output: Some(65_000),
            free: true,
            recommended: false,
            tier: "code",
        },
        ModelSpec {
            id: "groq-llama",
            name: "Llama (Groq fast)",
            provider: "groq",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "fast",
        },
        ModelSpec {
            id: "groq-qwen",
            name: "Qwen (Groq fast)",
            provider: "groq",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "fast",
        },
        ModelSpec {
            id: "groq-deepseek",
            name: "DeepSeek (Groq fast)",
            provider: "groq",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "fast",
        },
        ModelSpec {
            id: "nemotron",
            name: "Nemotron",
            provider: "nvidia",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "open",
        },
        ModelSpec {
            id: "kimi-k2-hf",
            name: "Kimi-K2 (HF)",
            provider: "huggingface",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "open",
        },
        ModelSpec {
            id: "glm-4.6-hf",
            name: "GLM-4.6 (HF)",
            provider: "huggingface",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "open",
        },
        ModelSpec {
            id: "openrouter:auto",
            name: "OpenRouter (aggregator, any model incl. `:free`)",
            provider: "openrouter",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "aggregator",
        },
        ModelSpec {
            id: "copilot-gpt-5x",
            name: "GPT-5.x (Copilot)",
            provider: "copilot",
            context_window: None,
            max_output: None,
            free: true,
            recommended: false,
            tier: "aggregator",
        },
        ModelSpec {
            id: "zen-gpt-5.1-codex",
            name: "GPT-5.1 Codex (Zen)",
            provider: "opencode-zen",
            context_window: None,
            max_output: None,
            free: false,
            recommended: false,
            tier: "aggregator",
        },
        ModelSpec {
            id: "zen-qwen3-coder",
            name: "Qwen3 Coder (Zen)",
            provider: "opencode-zen",
            context_window: None,
            max_output: None,
            free: false,
            recommended: false,
            tier: "aggregator",
        },
        ModelSpec {
            id: "go-coding",
            name: "Open coding models (Go)",
            provider: "opencode-go",
            context_window: None,
            max_output: None,
            free: false,
            recommended: false,
            tier: "aggregator",
        },
        ModelSpec {
            id: "302ai:auto",
            name: "302.AI (aggregate, CN + global)",
            provider: "302ai",
            context_window: None,
            max_output: None,
            free: false,
            recommended: false,
            tier: "aggregator",
        },
    ]
}

/// Look up a provider spec by id.
pub fn provider(id: &str) -> Option<&'static ProviderSpec> {
    providers().iter().find(|p| p.id == id)
}

/// All model entries whose canonical id matches (possibly several hosts).
pub fn find_models(id: &str) -> Vec<&'static ModelSpec> {
    models().iter().filter(|m| m.id == id).collect()
}

/// Models offered by a given provider.
pub fn models_for_provider(provider_id: &str) -> Vec<&'static ModelSpec> {
    models()
        .iter()
        .filter(|m| m.provider == provider_id)
        .collect()
}

/// The sheet's recommended list, in the sheet's order.
pub fn recommended_models() -> Vec<&'static ModelSpec> {
    models().iter().filter(|m| m.recommended).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn providers_are_unique_and_cover_models() {
        let ids: Vec<&str> = providers().iter().map(|p| p.id).collect();
        for p in providers() {
            assert_eq!(
                ids.iter().filter(|id| **id == p.id).count(),
                1,
                "dup provider {}",
                p.id
            );
        }
        for m in models() {
            assert!(
                provider(m.provider).is_some(),
                "model {} references unknown provider {}",
                m.id,
                m.provider
            );
        }
    }

    #[test]
    fn recommended_list_matches_sheet() {
        let rec: Vec<&str> = recommended_models().iter().map(|m| m.id).collect();
        assert_eq!(
            rec,
            vec![
                "gpt-5.2",
                "gpt-5.2-codex",
                "claude-opus-4.5",
                "claude-sonnet-4.5",
                "gemini-3-pro",
                "minimax-m2.1",
            ]
        );
    }

    #[test]
    fn token_windows_from_sheet_present_on_flagships() {
        for id in ["gpt-5.2", "claude-opus-4.5", "gemini-3-pro", "minimax-m2.1"] {
            let m = find_models(id);
            assert!(!m.is_empty());
            for spec in m {
                assert!(spec.context_window.is_some(), "{id} ctx missing");
                assert!(spec.max_output.is_some(), "{id} out missing");
            }
        }
        // Sheet says "varies" for these — must stay None (no invented numbers).
        for id in ["groq-llama", "openrouter:auto", "nemotron"] {
            for spec in find_models(id) {
                assert_eq!(spec.context_window, None, "{id} ctx must be varies");
            }
        }
    }

    #[test]
    fn qwen3_coder_is_hosted_by_two_providers() {
        let hosts = find_models("qwen3-coder-480b");
        assert_eq!(hosts.len(), 2);
        assert!(hosts.iter().any(|m| m.provider == "qwen"));
        assert!(hosts.iter().any(|m| m.provider == "cerebras"));
    }
}
