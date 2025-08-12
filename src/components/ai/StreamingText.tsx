import React, { useEffect, useState, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Pause, Play, Square, Zap, Clock } from 'lucide-react';
import { Button } from '../../ui/components/common';
import { Progress } from '../ui/progress';
import { Badge } from '../ui/badge';
import { useAIStreaming } from '../../hooks/useAI';

interface StreamingTextProps {
  text: string;
  isStreaming: boolean;
  canPause?: boolean;
  isPaused?: boolean;
  onPause?: () => void;
  onResume?: () => void;
  onStop?: () => void;
  onComplete?: (text: string) => void;
  className?: string;
  showControls?: boolean;
  showProgress?: boolean;
  estimatedDuration?: number; // in seconds
  wordsPerMinute?: number;
}

export const StreamingText: React.FC<StreamingTextProps> = ({
  text,
  isStreaming,
  canPause = true,
  isPaused = false,
  onPause,
  onResume,
  onStop,
  onComplete,
  className = '',
  showControls = true,
  showProgress = true,
  estimatedDuration: _estimatedDuration = 30,
  wordsPerMinute = 150,
}) => {
  const [displayedText, setDisplayedText] = useState('');
  const [currentWordIndex, setCurrentWordIndex] = useState(0);
  const [startTime, setStartTime] = useState<number | null>(null);
  const [elapsedTime, setElapsedTime] = useState(0);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const timeIntervalRef = useRef<NodeJS.Timeout | null>(null);
  
  const words = text.split(' ');
  const totalWords = words.length;
  const progress = totalWords > 0 ? (currentWordIndex / totalWords) * 100 : 0;
  const estimatedWordsPerSecond = wordsPerMinute / 60;
  const estimatedTimeRemaining = totalWords > 0 ? Math.max(0, (totalWords - currentWordIndex) / estimatedWordsPerSecond) : 0;
  
  // Start streaming animation
  useEffect(() => {
    if (isStreaming && !isPaused && text) {
      if (!startTime) {
        setStartTime(Date.now());
      }
      
      // Clear any existing interval
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
      
      // Calculate delay between words based on WPM
      const delayMs = (60 / wordsPerMinute) * 1000;
      
      intervalRef.current = setInterval(() => {
        setCurrentWordIndex(prev => {
          const nextIndex = prev + 1;
          
          if (nextIndex >= totalWords) {
            // Streaming complete
            if (intervalRef.current) {
              clearInterval(intervalRef.current);
              intervalRef.current = null;
            }
            onComplete?.(text);
            return totalWords;
          }
          
          return nextIndex;
        });
      }, delayMs);
    } else {
      // Pause or stop streaming
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    }
    
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isStreaming, isPaused, text, totalWords, wordsPerMinute, onComplete, startTime]);
  
  // Update displayed text based on current word index
  useEffect(() => {
    if (currentWordIndex > 0) {
      setDisplayedText(words.slice(0, currentWordIndex).join(' '));
    } else {
      setDisplayedText('');
    }
  }, [currentWordIndex, words]);
  
  // Track elapsed time
  useEffect(() => {
    if (isStreaming && !isPaused && startTime) {
      timeIntervalRef.current = setInterval(() => {
        setElapsedTime((Date.now() - startTime) / 1000);
      }, 100);
    } else {
      if (timeIntervalRef.current) {
        clearInterval(timeIntervalRef.current);
        timeIntervalRef.current = null;
      }
    }
    
    return () => {
      if (timeIntervalRef.current) {
        clearInterval(timeIntervalRef.current);
      }
    };
  }, [isStreaming, isPaused, startTime]);
  
  // Reset when text changes
  useEffect(() => {
    setCurrentWordIndex(0);
    setDisplayedText('');
    setStartTime(null);
    setElapsedTime(0);
  }, [text]);
  
  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
      if (timeIntervalRef.current) {
        clearInterval(timeIntervalRef.current);
      }
    };
  }, []);
  
  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };
  
  const currentWPM = elapsedTime > 0 ? Math.round((currentWordIndex / elapsedTime) * 60) : 0;
  
  return (
    <div className={`space-y-4 ${className}`}>
      {/* Streaming Controls */}
      {showControls && isStreaming && (
        <div className="flex items-center justify-between p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-700">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-2">
              <motion.div
                animate={isPaused ? {} : { scale: [1, 1.2, 1] }}
                transition={{ duration: 1, repeat: Infinity }}
                className="w-2 h-2 bg-blue-500 rounded-full"
              />
              <span className="text-sm font-medium text-blue-700 dark:text-blue-300">
                {isPaused ? 'Paused' : 'Generating...'}
              </span>
            </div>
            
            <div className="flex items-center gap-4 text-xs text-blue-600 dark:text-blue-400">
              <div className="flex items-center gap-1">
                <Clock className="w-3 h-3" />
                {formatTime(elapsedTime)}
              </div>
              <div className="flex items-center gap-1">
                <Zap className="w-3 h-3" />
                {currentWPM} WPM
              </div>
              {estimatedTimeRemaining > 0 && (
                <div>
                  ~{formatTime(estimatedTimeRemaining)} remaining
                </div>
              )}
            </div>
          </div>
          
          <div className="flex items-center gap-2">
            {canPause && (
              <Button
                size="sm"
                variant="outline"
                onClick={isPaused ? onResume : onPause}
                className="h-8"
              >
                {isPaused ? (
                  <>
                    <Play className="w-3 h-3 mr-1" />
                    Resume
                  </>
                ) : (
                  <>
                    <Pause className="w-3 h-3 mr-1" />
                    Pause
                  </>
                )}
              </Button>
            )}
            
            <Button
              size="sm"
              variant="destructive"
              onClick={onStop}
              className="h-8"
            >
              <Square className="w-3 h-3 mr-1" />
              Stop
            </Button>
          </div>
        </div>
      )}
      
      {/* Progress Bar */}
      {showProgress && isStreaming && totalWords > 0 && (
        <div className="space-y-2">
          <div className="flex items-center justify-between text-xs text-gray-600 dark:text-gray-400">
            <span>Progress</span>
            <span>{currentWordIndex} / {totalWords} words ({Math.round(progress)}%)</span>
          </div>
          <Progress value={progress} className="h-2" />
        </div>
      )}
      
      {/* Streaming Text Display */}
      <div className="relative">
        <div className="prose dark:prose-invert max-w-none">
          <div className="whitespace-pre-wrap leading-relaxed">
            {/* Already displayed text */}
            <span className="text-gray-900 dark:text-gray-100">
              {displayedText}
            </span>
            
            {/* Current word being typed */}
            {isStreaming && currentWordIndex < totalWords && (
              <motion.span
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="text-blue-600 dark:text-blue-400 font-medium"
              >
                {currentWordIndex > 0 ? ' ' : ''}
                {words[currentWordIndex]}
              </motion.span>
            )}
            
            {/* Typing cursor */}
            {isStreaming && !isPaused && (
              <motion.span
                animate={{ opacity: [1, 0, 1] }}
                transition={{ duration: 1, repeat: Infinity }}
                className="inline-block w-0.5 h-5 bg-blue-500 ml-1"
              />
            )}
            
            {/* Remaining text (dimmed) */}
            {currentWordIndex < totalWords && (
              <span className="text-gray-400 dark:text-gray-600">
                {currentWordIndex > 0 ? ' ' : ''}
                {words.slice(currentWordIndex + (isStreaming ? 1 : 0)).join(' ')}
              </span>
            )}
          </div>
        </div>
        
        {/* Completion indicator */}
        <AnimatePresence>
          {!isStreaming && currentWordIndex >= totalWords && totalWords > 0 && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              className="mt-4 flex items-center gap-2"
            >
              <Badge variant="secondary" className="text-green-600 dark:text-green-400">
                ✓ Complete
              </Badge>
              <span className="text-xs text-gray-500 dark:text-gray-400">
                Generated {totalWords} words in {formatTime(elapsedTime)}
              </span>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

// Wrapper component that integrates with the AI store
export const AIStreamingText: React.FC<{
  className?: string;
  showControls?: boolean;
  showProgress?: boolean;
  onComplete?: (text: string) => void;
}> = ({ className, showControls = true, showProgress = true, onComplete }) => {
  const { 
    streaming, 
    pauseStreaming, 
    resumeStreaming, 
    stopStreaming 
  } = useAIStreaming();
  
  return (
    <StreamingText
      text={streaming.currentText}
      isStreaming={streaming.isStreaming}
      canPause={streaming.canPause}
      isPaused={streaming.isPaused}
      onPause={pauseStreaming}
      onResume={resumeStreaming}
      onStop={stopStreaming}
      onComplete={onComplete}
      className={className}
      showControls={showControls}
      showProgress={showProgress}
    />
  );
};

export default StreamingText;