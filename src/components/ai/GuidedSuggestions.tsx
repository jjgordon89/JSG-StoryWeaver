import { useState, useCallback } from 'react';
import { useAIStore } from '../../stores/aiStore';
import './GuidedSuggestions.css';

export const GuidedSuggestions = () => {
  const [prompt, setPrompt] = useState('');
  const [suggestions, setSuggestions] = useState<string[]>([]);
  const { getGuidedSuggestions, isLoading, error } = useAIStore();

  const handleGetSuggestions = useCallback(async () => {
    if (!prompt) return;
    try {
      const result = await getGuidedSuggestions(prompt);
      setSuggestions(result);
    } catch (err) {
      // Error is already handled in the store, but you could add UI feedback here
      console.error(err);
    }
  }, [prompt, getGuidedSuggestions]);

  return (
    <div className="guided-suggestions">
      <h3>Guided Suggestions</h3>
      <textarea
        value={prompt}
        onChange={(e) => setPrompt(e.target.value)}
        placeholder="Enter a prompt to get suggestions..."
      />
      <button onClick={handleGetSuggestions} disabled={isLoading}>
        {isLoading ? 'Loading...' : 'Get Suggestions'}
      </button>
      {error && <p className="error">{error}</p>}
      {suggestions.length > 0 && (
        <ul>
          {suggestions.map((suggestion, index) => (
            <li key={index}>{suggestion}</li>
          ))}
        </ul>
      )}
    </div>
  );
};