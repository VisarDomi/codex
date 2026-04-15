use codex_utils_output_truncation::TruncationPolicy;

pub(crate) const DEFAULT_MAX_OUTPUT_TOKENS: usize = 10_000;

pub(crate) fn resolve_max_output_tokens(max_tokens: Option<usize>) -> usize {
    max_tokens.unwrap_or(DEFAULT_MAX_OUTPUT_TOKENS)
}

pub(crate) fn effective_tool_output_token_limit(
    requested_max_output_tokens: Option<usize>,
    configured_max_output_tokens: Option<usize>,
    truncation_policy: TruncationPolicy,
) -> usize {
    resolve_max_output_tokens(requested_max_output_tokens.or(configured_max_output_tokens))
        .min(truncation_policy.token_budget())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn effective_tool_output_token_limit_uses_configured_default() {
        assert_eq!(
            effective_tool_output_token_limit(None, Some(12_000), TruncationPolicy::Tokens(20_000)),
            12_000
        );
    }

    #[test]
    fn effective_tool_output_token_limit_prefers_request_over_configured_default() {
        assert_eq!(
            effective_tool_output_token_limit(
                Some(2_000),
                Some(12_000),
                TruncationPolicy::Tokens(20_000)
            ),
            2_000
        );
    }

    #[test]
    fn effective_tool_output_token_limit_clamps_to_truncation_policy() {
        assert_eq!(
            effective_tool_output_token_limit(None, Some(12_000), TruncationPolicy::Tokens(4_000)),
            4_000
        );
    }
}
