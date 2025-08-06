import React, { useEffect, useState } from 'react';
import { useSettingsStore } from '../../stores/settingsStore';

const FocusModeHint: React.FC = () => {
  const { focusModeEnabled } = useSettingsStore();
  const [visible, setVisible] = useState(false);

  // Show the hint briefly when focus mode is toggled
  useEffect(() => {
    if (focusModeEnabled) {
      setVisible(true);
      const timer = setTimeout(() => {
        setVisible(false);
      }, 5000); // Hide after 5 seconds
      
      return () => clearTimeout(timer);
    }
  }, [focusModeEnabled]);

  if (!visible) return null;

  return (
    <div className="focus-mode-hint">
      Press <kbd className="px-1 py-0.5 bg-gray-700 rounded">Ctrl+Shift+F</kbd> to exit focus mode
    </div>
  );
};

export default FocusModeHint;
