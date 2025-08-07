import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Sparkles, RefreshCw, X } from 'lucide-react';
import { useAI } from '../../hooks/useAI';

interface RelatedWordsProps {
  word: string;
  position: { x: number; y: number };
  onClose: () => void;
  onWordSelect: (word: string) => void;
  context?: string;
}

const RelatedWords: React.FC<RelatedWordsProps> = ({
  word,
  position,
  onClose,
  onWordSelect,
  context = ''
}) => {
  const [relatedWords, setRelatedWords] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { getRelatedWords } = useAI();

  // Load related words when component mounts
  useEffect(() => {
    loadRelatedWords();
  }, [word]);

  const loadRelatedWords = async () => {
    if (!word.trim()) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const words = await getRelatedWords(word, context);
      setRelatedWords(words.filter(w => w.toLowerCase() !== word.toLowerCase()));
    } catch (err) {
      console.error('Error loading related words:', err);
      setError('Failed to load related words');
    } finally {
      setIsLoading(false);
    }
  };

  const handleWordClick = (selectedWord: string) => {
    onWordSelect(selectedWord);
    onClose();
  };

  const handleRefresh = () => {
    loadRelatedWords();
  };

  // Show initial quick suggestions (first 6 words)
  const quickSuggestions = relatedWords.slice(0, 6);
  const allWords = relatedWords;

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, scale: 0.9, y: -10 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.9, y: -10 }}
        transition={{ duration: 0.2 }}
        className="fixed z-50 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 min-w-64 max-w-96"
        style={{
          left: Math.min(position.x, window.innerWidth - 400),
          top: Math.min(position.y + 10, window.innerHeight - 300),
        }}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-3 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center gap-2">
            <Sparkles className="w-4 h-4 text-purple-500" />
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              Related to "{word}"
            </span>
          </div>
          <div className="flex items-center gap-1">
            <button
              onClick={handleRefresh}
              disabled={isLoading}
              className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              title="Refresh suggestions"
            >
              <RefreshCw className={`w-4 h-4 text-gray-500 ${isLoading ? 'animate-spin' : ''}`} />
            </button>
            <button
              onClick={onClose}
              className="p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              title="Close"
            >
              <X className="w-4 h-4 text-gray-500" />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-3">
          {isLoading && (
            <div className="flex items-center justify-center py-8">
              <div className="flex items-center gap-2 text-gray-500">
                <RefreshCw className="w-4 h-4 animate-spin" />
                <span className="text-sm">Finding related words...</span>
              </div>
            </div>
          )}

          {error && (
            <div className="text-center py-4">
              <p className="text-sm text-red-500 mb-2">{error}</p>
              <button
                onClick={handleRefresh}
                className="text-xs text-blue-500 hover:text-blue-600 underline"
              >
                Try again
              </button>
            </div>
          )}

          {!isLoading && !error && relatedWords.length > 0 && (
            <div className="space-y-3">
              {/* Quick suggestions */}
              {!isExpanded && (
                <div>
                  <div className="grid grid-cols-2 gap-2">
                    {quickSuggestions.map((relatedWord, index) => (
                      <button
                        key={index}
                        onClick={() => handleWordClick(relatedWord)}
                        className="px-3 py-2 text-sm bg-gray-50 dark:bg-gray-700 hover:bg-purple-50 dark:hover:bg-purple-900/20 text-gray-700 dark:text-gray-300 hover:text-purple-700 dark:hover:text-purple-300 rounded-md transition-colors text-left"
                      >
                        {relatedWord}
                      </button>
                    ))}
                  </div>
                  
                  {allWords.length > 6 && (
                    <button
                      onClick={() => setIsExpanded(true)}
                      className="w-full mt-3 px-3 py-2 text-sm text-purple-600 dark:text-purple-400 hover:text-purple-700 dark:hover:text-purple-300 border border-purple-200 dark:border-purple-700 hover:border-purple-300 dark:hover:border-purple-600 rounded-md transition-colors"
                    >
                      Show all {allWords.length} words
                    </button>
                  )}
                </div>
              )}

              {/* Expanded word cloud */}
              {isExpanded && (
                <div>
                  <div className="flex items-center justify-between mb-3">
                    <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
                      All Related Words ({allWords.length})
                    </span>
                    <button
                      onClick={() => setIsExpanded(false)}
                      className="text-xs text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                    >
                      Show less
                    </button>
                  </div>
                  
                  <div className="max-h-64 overflow-y-auto">
                    <div className="flex flex-wrap gap-2">
                      {allWords.map((relatedWord, index) => (
                        <button
                          key={index}
                          onClick={() => handleWordClick(relatedWord)}
                          className="px-2 py-1 text-sm bg-gray-50 dark:bg-gray-700 hover:bg-purple-50 dark:hover:bg-purple-900/20 text-gray-700 dark:text-gray-300 hover:text-purple-700 dark:hover:text-purple-300 rounded transition-colors"
                          style={{
                            fontSize: `${Math.max(0.75, 1 - (index * 0.02))}rem`
                          }}
                        >
                          {relatedWord}
                        </button>
                      ))}
                    </div>
                  </div>
                </div>
              )}
            </div>
          )}

          {!isLoading && !error && relatedWords.length === 0 && (
            <div className="text-center py-4">
              <p className="text-sm text-gray-500">No related words found</p>
              <button
                onClick={handleRefresh}
                className="text-xs text-blue-500 hover:text-blue-600 underline mt-1"
              >
                Try again
              </button>
            </div>
          )}
        </div>

        {/* Footer tip */}
        {!isLoading && !error && relatedWords.length > 0 && (
          <div className="px-3 py-2 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-750 rounded-b-lg">
            <p className="text-xs text-gray-500 dark:text-gray-400">
              Click any word to replace "{word}"
            </p>
          </div>
        )}
      </motion.div>
    </AnimatePresence>
  );
};

export default RelatedWords;