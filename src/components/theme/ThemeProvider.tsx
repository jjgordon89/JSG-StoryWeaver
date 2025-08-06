import React, { createContext, useContext, useEffect, useState } from 'react';

type Theme = 'light' | 'dark' | 'system';

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
  isDark: boolean;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

interface ThemeProviderProps {
  children: React.ReactNode;
  defaultTheme?: Theme;
  storageKey?: string;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({
  children,
  defaultTheme = 'system',
  storageKey = 'storyweaver-theme',
}) => {
  const [theme, setTheme] = useState<Theme>(() => {
    // Try to get the theme from local storage
    const storedTheme = localStorage.getItem(storageKey);
    if (storedTheme && (storedTheme === 'light' || storedTheme === 'dark' || storedTheme === 'system')) {
      return storedTheme as Theme;
    }
    return defaultTheme;
  });

  const [isDark, setIsDark] = useState<boolean>(false);

  useEffect(() => {
    const root = window.document.documentElement;
    
    // Remove all theme classes
    root.classList.remove('light', 'dark');
    
    // Determine if we should use dark mode
    let isDarkMode = false;
    
    if (theme === 'system') {
      // Check system preference
      isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
    } else {
      isDarkMode = theme === 'dark';
    }
    
    // Apply the appropriate class
    root.classList.add(isDarkMode ? 'dark' : 'light');
    setIsDark(isDarkMode);
    
    // Store the theme preference
    localStorage.setItem(storageKey, theme);
  }, [theme, storageKey]);

  // Listen for system theme changes if using system theme
  useEffect(() => {
    if (theme !== 'system') return;
    
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    
    const handleChange = () => {
      setIsDark(mediaQuery.matches);
      document.documentElement.classList.remove('light', 'dark');
      document.documentElement.classList.add(mediaQuery.matches ? 'dark' : 'light');
    };
    
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, [theme]);

  const value = {
    theme,
    setTheme,
    isDark,
  };

  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = (): ThemeContextType => {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};
