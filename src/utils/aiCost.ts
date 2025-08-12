/**
 * AI Cost Estimation Utilities
 * 
 * Goal: Provide lightweight, dependency-free estimators for tokens and "credits".
 * - Tokens are approximated from character length (avg ~4 chars per token).
 * - Credits are derived from tokens using a simple heuristic aligned with current code patterns.
 *
 * Notes:
 * - These are estimates intended for pre-execution UX (badges/tooltips).
 * - For streaming, show a range or use conservative defaults.
 * - The application currently treats "credits" as a project-specific consumption unit,
 *   not currency. Aligning with the existing streaming code, we use a simple ratio.
 */

/**
 * Roughly estimate tokens from text length.
 * Uses a 4 chars/token heuristic (typical for English prose).
 */
export function estimateTokensFromText(text: string | undefined | null): number {
  if (!text) return 0;
  const trimmed = text.trim();
  if (!trimmed) return 0;
  return Math.max(0, Math.ceil(trimmed.length / 4));
}

/**
 * Map write card length to approximate output tokens per card.
 */
export function estimateWriteTokensPerCard(cardLength?: 'short' | 'medium' | 'long'): number {
  const map: Record<'short' | 'medium' | 'long', number> = {
    short: 150,
    medium: 300,
    long: 600,
  };
  return map[(cardLength || 'medium') as 'short' | 'medium' | 'long'] ?? 300;
}

/**
 * Estimate expected output tokens for the Write tool.
 * Multiplies per-card token estimate by requested card count.
 */
export function estimateExpectedOutputTokensForWrite(options: {
  card_length?: 'short' | 'medium' | 'long';
  card_count?: number;
}): number {
  const perCard = estimateWriteTokensPerCard(options.card_length);
  const count = typeof options.card_count === 'number' ? Math.max(1, options.card_count) : 1;
  return perCard * count;
}

/**
 * Estimate expected output tokens for Expand tool.
 * Applies a multiplier to the input tokens (default 2x).
 */
export function estimateExpectedOutputTokensForExpand(inputTokens: number, lengthMultiplier?: number): number {
  const mult = typeof lengthMultiplier === 'number' && lengthMultiplier > 0 ? lengthMultiplier : 2;
  return Math.ceil(inputTokens * mult);
}

/**
 * Convert tokens to "credits" using a simple project-local heuristic.
 * Current pattern observed in streaming code: credits ~= chars / 10.
 * Assuming ~4 chars/token, that's roughly tokens / 2.5. We keep it simple here:
 * 
 * credits = ceil(totalTokens / 10)
 * 
 * This is intentionally conservative and easy to reason about. Adjust as the
 * project refines its credit economy.
 */
export function estimateCreditsFromTokens(totalTokens: number): number {
  if (totalTokens <= 0) return 0;
  return Math.ceil(totalTokens / 10);
}

/**
 * Estimate credits for an operation from input and expected output tokens.
 */
export function estimateOperationCredits(inputTokens: number, expectedOutputTokens: number): number {
  const total = Math.max(0, inputTokens) + Math.max(0, expectedOutputTokens);
  return estimateCreditsFromTokens(total);
}

/**
 * Model-based credit multipliers (very rough heuristic).
 * Lower-cost models apply a discount; higher-cost models may apply a premium.
 */
export function getModelCreditMultiplier(model?: string): number {
  if (!model) return 1.0;
  const m = model.toLowerCase();

  // OpenAI family examples
  if (m.includes('gpt-4o')) return 0.85;
  if (m.includes('gpt-4-turbo') || m.includes('gpt-4.1') || m.includes('gpt-4o-mini')) return 0.9;
  if (m.includes('gpt-4')) return 1.0;

  // Anthropic examples
  if (m.includes('claude-3-opus')) return 1.1;
  if (m.includes('claude')) return 1.0;

  // Open/local models (generally cheaper)
  if (m.includes('llama') || m.includes('mistral') || m.includes('mixtral')) return 0.6;

  return 1.0;
}

/**
 * Estimate credits and apply a model multiplier.
 */
export function estimateOperationCreditsWithModel(
  inputTokens: number,
  expectedOutputTokens: number,
  model?: string
): number {
  const base = estimateOperationCredits(inputTokens, expectedOutputTokens);
  const mult = getModelCreditMultiplier(model);
  return Math.max(0, Math.ceil(base * mult));
}
