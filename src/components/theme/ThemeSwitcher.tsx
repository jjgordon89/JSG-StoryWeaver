import React from 'react';
import { useTheme } from './ThemeProvider';
import { Button } from '../../ui/components/common';
import { DropdownButton } from '../ui/Dropdown';

type Theme = 'light' | 'dark' | 'system';

interface ThemeSwitcherProps {
  variant?: 'button' | 'dropdown';
  className?: string;
}

export const ThemeSwitcher: React.FC<ThemeSwitcherProps> = ({
  variant = 'dropdown',
  className = '',
}) => {
  const { theme, setTheme, isDark } = useTheme();

  // Icons for the themes
  const LightIcon = () => (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="h-4 w-4"
    >
      <circle cx="12" cy="12" r="5" />
      <line x1="12" y1="1" x2="12" y2="3" />
      <line x1="12" y1="21" x2="12" y2="23" />
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
      <line x1="1" y1="12" x2="3" y2="12" />
      <line x1="21" y1="12" x2="23" y2="12" />
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
    </svg>
  );

  const DarkIcon = () => (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="h-4 w-4"
    >
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
    </svg>
  );

  const SystemIcon = () => (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className="h-4 w-4"
    >
      <rect x="2" y="3" width="20" height="14" rx="2" ry="2" />
      <line x1="8" y1="21" x2="16" y2="21" />
      <line x1="12" y1="17" x2="12" y2="21" />
    </svg>
  );

  // For the button variant, we cycle through themes
  const cycleTheme = () => {
    const themes: Theme[] = ['light', 'dark', 'system'];
    const currentIndex = themes.indexOf(theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    setTheme(themes[nextIndex]);
  };

  if (variant === 'button') {
    return (
      <Button
        variant="outline"
        size="sm"
        className={`p-2 ${className}`}
        onClick={cycleTheme}
        aria-label="Toggle theme"
      >
        {theme === 'light' ? <LightIcon /> : theme === 'dark' ? <DarkIcon /> : <SystemIcon />}
      </Button>
    );
  }

  return (
    <DropdownButton
      label="Theme"
      icon={theme === 'light' ? <LightIcon /> : theme === 'dark' ? <DarkIcon /> : <SystemIcon />}
      items={[
        { label: 'Light', value: 'light', selected: theme === 'light' },
        { label: 'Dark', value: 'dark', selected: theme === 'dark' },
        { label: 'System', value: 'system', selected: theme === 'system' },
      ]}
      onSelect={(value) => setTheme(value as Theme)}
      variant="outline"
      size="sm"
      className={className}
    />
  );
};
