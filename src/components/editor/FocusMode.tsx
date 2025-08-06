import React, { useEffect } from 'react';
import { useSettingsStore } from '../../stores/settingsStore';
import FocusModeHint from './FocusModeHint';

interface FocusModeProps {
  children: React.ReactNode;
}

const FocusMode: React.FC<FocusModeProps> = ({ children }) => {
  const { 
    focusModeEnabled, 
    focusModeOptions,
    toggleFocusMode 
  } = useSettingsStore();

  // Handle keyboard shortcut for toggling focus mode (Ctrl+Shift+F or Cmd+Shift+F)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'f') {
        e.preventDefault();
        toggleFocusMode();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [toggleFocusMode]);

  // Apply focus mode styles to the document body
  useEffect(() => {
    if (focusModeEnabled) {
      document.body.classList.add('focus-mode');
      if (focusModeOptions.dimUI) {
        document.body.classList.add('focus-mode-dim');
      }
    } else {
      document.body.classList.remove('focus-mode');
      document.body.classList.remove('focus-mode-dim');
    }
  }, [focusModeEnabled, focusModeOptions.dimUI]);

  return (
    <>
      {/* Focus mode toggle button */}
      <button
        onClick={toggleFocusMode}
        className="focus-mode-toggle absolute top-4 right-4 z-50 p-2 rounded-full bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
        title={focusModeEnabled ? "Exit Focus Mode (Ctrl+Shift+F)" : "Enter Focus Mode (Ctrl+Shift+F)"}
      >
        {focusModeEnabled ? (
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M9 9l6 6M15 9l-6 6"></path>
          </svg>
        ) : (
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3"></path>
          </svg>
        )}
      </button>

      {/* Render children with focus mode classes */}
      <div className={`focus-mode-container ${focusModeEnabled ? 'is-focused' : ''}`}>
        {children}
      </div>

      {/* Keyboard shortcut hint */}
      <FocusModeHint />
    </>
  );
};

export default FocusMode;
