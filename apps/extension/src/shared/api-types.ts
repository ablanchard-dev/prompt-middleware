export interface OptimizeRequest {
  raw_user_input: string;
  target_platform: "chatgpt" | "claude" | "gemini" | "deepseek" | "unknown";
  language: "fr" | "en" | "auto";
  mode: "preview" | "replace";
  user_preferences: {
    tone: string | null;
    detail_level: "short" | "normal" | "detailed" | "expert";
  };
}

export interface OptimizeResponse {
  optimized_prompt: string;
  detected_language: string;
  detected_domain: string;
  detected_intent: string;
  confidence: number;
  quality_score: {
    clarity: number;
    context: number;
    constraints: number;
    format: number;
    overall: number;
  };
  warnings: string[];
  needs_clarification: boolean;
  clarification_questions: string[];
}

