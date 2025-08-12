import React, { useState, useEffect } from 'react';
import { Card, CardHeader, CardContent } from '../../ui/components/common';
import { Button } from '../../ui/components/common';
import { ThemeSwitcher } from './ThemeSwitcher';

interface AccessibilitySettingsProps {
  className?: string;
}

export const AccessibilitySettings: React.FC<AccessibilitySettingsProps> = ({
  className = '',
}) => {
  // Font size settings
  const [fontSize, setFontSize] = useState<number>(() => {
    const storedSize = localStorage.getItem('storyweaver-font-size');
    return storedSize ? parseInt(storedSize, 10) : 16; // Default to 16px
  });

  // Line height settings
  const [lineHeight, setLineHeight] = useState<number>(() => {
    const storedHeight = localStorage.getItem('storyweaver-line-height');
    return storedHeight ? parseFloat(storedHeight) : 1.5; // Default to 1.5
  });

  // Reduced motion preference
  const [reducedMotion, setReducedMotion] = useState<boolean>(() => {
    const storedPref = localStorage.getItem('storyweaver-reduced-motion');
    return storedPref ? storedPref === 'true' : false;
  });

  // High contrast mode
  const [highContrast, setHighContrast] = useState<boolean>(() => {
    const storedPref = localStorage.getItem('storyweaver-high-contrast');
    return storedPref ? storedPref === 'true' : false;
  });

  // Apply font size changes
  useEffect(() => {
    document.documentElement.style.setProperty('--base-font-size', `${fontSize}px`);
    localStorage.setItem('storyweaver-font-size', fontSize.toString());
  }, [fontSize]);

  // Apply line height changes
  useEffect(() => {
    document.documentElement.style.setProperty('--base-line-height', lineHeight.toString());
    localStorage.setItem('storyweaver-line-height', lineHeight.toString());
  }, [lineHeight]);

  // Apply reduced motion preference
  useEffect(() => {
    if (reducedMotion) {
      document.documentElement.classList.add('reduce-motion');
    } else {
      document.documentElement.classList.remove('reduce-motion');
    }
    localStorage.setItem('storyweaver-reduced-motion', reducedMotion.toString());
  }, [reducedMotion]);

  // Apply high contrast mode
  useEffect(() => {
    if (highContrast) {
      document.documentElement.classList.add('high-contrast');
    } else {
      document.documentElement.classList.remove('high-contrast');
    }
    localStorage.setItem('storyweaver-high-contrast', highContrast.toString());
  }, [highContrast]);

  // Reset all settings to defaults
  const resetToDefaults = () => {
    setFontSize(16);
    setLineHeight(1.5);
    setReducedMotion(false);
    setHighContrast(false);
  };

  return (
    <Card className={`accessibility-settings ${className}`}>
      <CardHeader className="flex flex-row items-center justify-between">
        <h2 className="text-xl font-semibold">Accessibility Settings</h2>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <h3 className="text-lg font-medium">Theme</h3>
          <div className="flex items-center space-x-4">
            <ThemeSwitcher variant="dropdown" />
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="text-lg font-medium">Font Size</h3>
          <div className="flex items-center space-x-4">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setFontSize(Math.max(12, fontSize - 1))}
              aria-label="Decrease font size"
            >
              A-
            </Button>
            <span className="text-sm">{fontSize}px</span>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setFontSize(Math.min(24, fontSize + 1))}
              aria-label="Increase font size"
            >
              A+
            </Button>
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="text-lg font-medium">Line Height</h3>
          <div className="flex items-center space-x-4">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setLineHeight(Math.max(1.0, lineHeight - 0.1))}
              aria-label="Decrease line height"
            >
              -
            </Button>
            <span className="text-sm">{lineHeight.toFixed(1)}</span>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setLineHeight(Math.min(2.5, lineHeight + 0.1))}
              aria-label="Increase line height"
            >
              +
            </Button>
          </div>
        </div>

        <div className="space-y-2">
          <h3 className="text-lg font-medium">Motion & Contrast</h3>
          <div className="flex flex-col space-y-2">
            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={reducedMotion}
                onChange={(e) => setReducedMotion(e.target.checked)}
                className="rounded border-slate-300 dark:border-slate-700"
              />
              <span>Reduce motion</span>
            </label>
            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={highContrast}
                onChange={(e) => setHighContrast(e.target.checked)}
                className="rounded border-slate-300 dark:border-slate-700"
              />
              <span>High contrast mode</span>
            </label>
          </div>
        </div>

        <div className="pt-4">
          <Button
            variant="outline"
            size="sm"
            onClick={resetToDefaults}
            className="text-slate-600 dark:text-slate-400"
          >
            Reset to Defaults
          </Button>
        </div>
      </CardContent>
    </Card>
  );
};
