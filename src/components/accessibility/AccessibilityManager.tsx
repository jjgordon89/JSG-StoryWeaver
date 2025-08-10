/**
 * Accessibility Manager for UI/UX refinements in Phase 6
 * Handles screen reader support, keyboard navigation, focus management, and color contrast
 */

import React, { createContext, useContext, useEffect, useState, useCallback, useRef } from 'react';
import { toast } from 'react-hot-toast';

// Accessibility context
interface AccessibilityContextType {
  isScreenReaderActive: boolean;
  highContrastMode: boolean;
  reducedMotion: boolean;
  fontSize: 'small' | 'medium' | 'large' | 'extra-large';
  keyboardNavigation: boolean;
  focusVisible: boolean;
  announceToScreenReader: (message: string, priority?: 'polite' | 'assertive') => void;
  setHighContrastMode: (enabled: boolean) => void;
  setReducedMotion: (enabled: boolean) => void;
  setFontSize: (size: 'small' | 'medium' | 'large' | 'extra-large') => void;
  trapFocus: (element: HTMLElement) => () => void;
  skipToContent: () => void;
  cycleFocusRegions: () => void;
}

const AccessibilityContext = createContext<AccessibilityContextType | null>(null);

// Hook to use accessibility context
export const useAccessibility = () => {
  const context = useContext(AccessibilityContext);
  if (!context) {
    throw new Error('useAccessibility must be used within an AccessibilityProvider');
  }
  return context;
};

// Keyboard shortcuts configuration
const KEYBOARD_SHORTCUTS = {
  QUICK_TOOLS: 'ctrl+k',
  CYCLE_FOCUS: 'F6',
  SKIP_TO_CONTENT: 'ctrl+shift+c',
  TOGGLE_HIGH_CONTRAST: 'ctrl+shift+h',
  INCREASE_FONT: 'ctrl+plus',
  DECREASE_FONT: 'ctrl+minus',
  RESET_FONT: 'ctrl+0',
  ESCAPE: 'Escape',
} as const;

// Focus regions for F6 cycling
const FOCUS_REGIONS = [
  '[data-focus-region="navigation"]',
  '[data-focus-region="main-content"]',
  '[data-focus-region="sidebar"]',
  '[data-focus-region="toolbar"]',
  '[data-focus-region="footer"]',
] as const;

// Color contrast checker
class ColorContrastChecker {
  /**
   * Calculate relative luminance of a color
   */
  private static getRelativeLuminance(r: number, g: number, b: number): number {
    const [rs, gs, bs] = [r, g, b].map(c => {
      c = c / 255;
      return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
    });
    return 0.2126 * rs + 0.7152 * gs + 0.0722 * bs;
  }

  /**
   * Calculate contrast ratio between two colors
   */
  static getContrastRatio(color1: string, color2: string): number {
    const rgb1 = this.hexToRgb(color1);
    const rgb2 = this.hexToRgb(color2);
    
    if (!rgb1 || !rgb2) return 0;
    
    const l1 = this.getRelativeLuminance(rgb1.r, rgb1.g, rgb1.b);
    const l2 = this.getRelativeLuminance(rgb2.r, rgb2.g, rgb2.b);
    
    const lighter = Math.max(l1, l2);
    const darker = Math.min(l1, l2);
    
    return (lighter + 0.05) / (darker + 0.05);
  }

  /**
   * Convert hex color to RGB
   */
  private static hexToRgb(hex: string): { r: number; g: number; b: number } | null {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? {
      r: parseInt(result[1], 16),
      g: parseInt(result[2], 16),
      b: parseInt(result[3], 16)
    } : null;
  }

  /**
   * Check if contrast ratio meets WCAG standards
   */
  static meetsWCAGStandards(ratio: number, level: 'AA' | 'AAA' = 'AA', isLargeText = false): boolean {
    if (level === 'AA') {
      return isLargeText ? ratio >= 3 : ratio >= 4.5;
    } else {
      return isLargeText ? ratio >= 4.5 : ratio >= 7;
    }
  }

  /**
   * Analyze all color combinations on the page
   */
  static analyzePageContrast(): Array<{
    element: HTMLElement;
    foreground: string;
    background: string;
    ratio: number;
    passes: boolean;
    recommendation?: string;
  }> {
    const results: Array<{
      element: HTMLElement;
      foreground: string;
      background: string;
      ratio: number;
      passes: boolean;
      recommendation?: string;
    }> = [];

    const elements = document.querySelectorAll('*');
    
    elements.forEach(element => {
      const htmlElement = element as HTMLElement;
      const styles = window.getComputedStyle(htmlElement);
      const color = styles.color;
      const backgroundColor = styles.backgroundColor;
      
      // Skip elements without text or transparent backgrounds
      if (!color || !backgroundColor || backgroundColor === 'rgba(0, 0, 0, 0)') {
        return;
      }
      
      // Convert colors to hex (simplified - would need more robust conversion)
      const foregroundHex = this.rgbToHex(color);
      const backgroundHex = this.rgbToHex(backgroundColor);
      
      if (foregroundHex && backgroundHex) {
        const ratio = this.getContrastRatio(foregroundHex, backgroundHex);
        const fontSize = parseFloat(styles.fontSize);
        const isLargeText = fontSize >= 18 || (fontSize >= 14 && styles.fontWeight === 'bold');
        const passes = this.meetsWCAGStandards(ratio, 'AA', isLargeText);
        
        if (!passes) {
          results.push({
            element: htmlElement,
            foreground: foregroundHex,
            background: backgroundHex,
            ratio,
            passes,
            recommendation: ratio < 3 ? 'Critical: Increase contrast significantly' :
                          ratio < 4.5 ? 'Moderate: Increase contrast for better readability' :
                          'Minor: Consider slight contrast improvement'
          });
        }
      }
    });
    
    return results;
  }

  /**
   * Convert RGB string to hex (simplified)
   */
  private static rgbToHex(rgb: string): string | null {
    const match = rgb.match(/rgb\((\d+),\s*(\d+),\s*(\d+)\)/);
    if (!match) return null;
    
    const [, r, g, b] = match;
    return `#${parseInt(r).toString(16).padStart(2, '0')}${parseInt(g).toString(16).padStart(2, '0')}${parseInt(b).toString(16).padStart(2, '0')}`;
  }
}

// Focus trap utility
class FocusTrap {
  private element: HTMLElement;
  private previousActiveElement: Element | null;
  private focusableElements: HTMLElement[];
  private firstFocusableElement: HTMLElement | null;
  private lastFocusableElement: HTMLElement | null;

  constructor(element: HTMLElement) {
    this.element = element;
    this.previousActiveElement = document.activeElement;
    this.focusableElements = this.getFocusableElements();
    this.firstFocusableElement = this.focusableElements[0] || null;
    this.lastFocusableElement = this.focusableElements[this.focusableElements.length - 1] || null;
    
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.activate();
  }

  private getFocusableElements(): HTMLElement[] {
    const focusableSelectors = [
      'a[href]',
      'button:not([disabled])',
      'textarea:not([disabled])',
      'input:not([disabled])',
      'select:not([disabled])',
      '[tabindex]:not([tabindex="-1"])',
      '[contenteditable="true"]'
    ].join(', ');
    
    return Array.from(this.element.querySelectorAll(focusableSelectors)) as HTMLElement[];
  }

  private handleKeyDown(event: KeyboardEvent) {
    if (event.key !== 'Tab') return;
    
    if (this.focusableElements.length === 0) {
      event.preventDefault();
      return;
    }
    
    if (event.shiftKey) {
      // Shift + Tab
      if (document.activeElement === this.firstFocusableElement) {
        event.preventDefault();
        this.lastFocusableElement?.focus();
      }
    } else {
      // Tab
      if (document.activeElement === this.lastFocusableElement) {
        event.preventDefault();
        this.firstFocusableElement?.focus();
      }
    }
  }

  private activate() {
    this.element.addEventListener('keydown', this.handleKeyDown);
    this.firstFocusableElement?.focus();
  }

  public deactivate() {
    this.element.removeEventListener('keydown', this.handleKeyDown);
    if (this.previousActiveElement instanceof HTMLElement) {
      this.previousActiveElement.focus();
    }
  }
}

// Screen reader announcer
class ScreenReaderAnnouncer {
  private static instance: ScreenReaderAnnouncer;
  private politeRegion: HTMLElement;
  private assertiveRegion: HTMLElement;

  private constructor() {
    this.politeRegion = this.createAriaLiveRegion('polite');
    this.assertiveRegion = this.createAriaLiveRegion('assertive');
  }

  static getInstance(): ScreenReaderAnnouncer {
    if (!ScreenReaderAnnouncer.instance) {
      ScreenReaderAnnouncer.instance = new ScreenReaderAnnouncer();
    }
    return ScreenReaderAnnouncer.instance;
  }

  private createAriaLiveRegion(priority: 'polite' | 'assertive'): HTMLElement {
    const region = document.createElement('div');
    region.setAttribute('aria-live', priority);
    region.setAttribute('aria-atomic', 'true');
    region.style.position = 'absolute';
    region.style.left = '-10000px';
    region.style.width = '1px';
    region.style.height = '1px';
    region.style.overflow = 'hidden';
    document.body.appendChild(region);
    return region;
  }

  announce(message: string, priority: 'polite' | 'assertive' = 'polite') {
    const region = priority === 'polite' ? this.politeRegion : this.assertiveRegion;
    
    // Clear the region first to ensure the message is announced
    region.textContent = '';
    
    // Use a small delay to ensure screen readers pick up the change
    setTimeout(() => {
      region.textContent = message;
    }, 100);
    
    // Clear the message after a delay to prevent it from being read again
    setTimeout(() => {
      region.textContent = '';
    }, 1000);
  }
}

// Main Accessibility Manager Component
interface AccessibilityManagerProps {
  children: React.ReactNode;
}

export const AccessibilityManager: React.FC<AccessibilityManagerProps> = ({ children }) => {
  const [isScreenReaderActive, setIsScreenReaderActive] = useState(false);
  const [highContrastMode, setHighContrastMode] = useState(false);
  const [reducedMotion, setReducedMotion] = useState(false);
  const [fontSize, setFontSize] = useState<'small' | 'medium' | 'large' | 'extra-large'>('medium');
  const [keyboardNavigation, setKeyboardNavigation] = useState(false);
  const [focusVisible, setFocusVisible] = useState(false);
  const [currentFocusRegion, setCurrentFocusRegion] = useState(0);
  
  const screenReaderRef = useRef<ScreenReaderAnnouncer>();
  const activeFocusTraps = useRef<Set<FocusTrap>>(new Set());

  // Initialize screen reader announcer
  useEffect(() => {
    screenReaderRef.current = ScreenReaderAnnouncer.getInstance();
  }, []);

  // Detect screen reader usage
  useEffect(() => {
    const detectScreenReader = () => {
      // Check for common screen reader indicators
      const hasAriaLive = document.querySelector('[aria-live]');
      const hasScreenReaderText = document.querySelector('.sr-only, .screen-reader-text');
      const userAgent = navigator.userAgent.toLowerCase();
      const hasScreenReaderUA = userAgent.includes('nvda') || userAgent.includes('jaws') || userAgent.includes('dragon');
      
      setIsScreenReaderActive(!!(hasAriaLive || hasScreenReaderText || hasScreenReaderUA));
    };

    detectScreenReader();
    
    // Listen for focus events to detect keyboard navigation
    const handleFocusIn = () => {
      setKeyboardNavigation(true);
      setFocusVisible(true);
    };
    
    const handleMouseDown = () => {
      setFocusVisible(false);
    };
    
    document.addEventListener('focusin', handleFocusIn);
    document.addEventListener('mousedown', handleMouseDown);
    
    return () => {
      document.removeEventListener('focusin', handleFocusIn);
      document.removeEventListener('mousedown', handleMouseDown);
    };
  }, []);

  // Detect user preferences
  useEffect(() => {
    // Check for reduced motion preference
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    setReducedMotion(mediaQuery.matches);
    
    const handleChange = (e: MediaQueryListEvent) => {
      setReducedMotion(e.matches);
    };
    
    mediaQuery.addEventListener('change', handleChange);
    
    // Load saved preferences
    const savedHighContrast = localStorage.getItem('accessibility-high-contrast') === 'true';
    const savedFontSize = localStorage.getItem('accessibility-font-size') as typeof fontSize || 'medium';
    
    setHighContrastMode(savedHighContrast);
    setFontSize(savedFontSize);
    
    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  }, []);

  // Apply accessibility styles
  useEffect(() => {
    const root = document.documentElement;
    
    // High contrast mode
    if (highContrastMode) {
      root.classList.add('high-contrast');
    } else {
      root.classList.remove('high-contrast');
    }
    
    // Font size
    root.classList.remove('font-small', 'font-medium', 'font-large', 'font-extra-large');
    root.classList.add(`font-${fontSize}`);
    
    // Reduced motion
    if (reducedMotion) {
      root.classList.add('reduced-motion');
    } else {
      root.classList.remove('reduced-motion');
    }
    
    // Focus visible
    if (focusVisible) {
      root.classList.add('focus-visible');
    } else {
      root.classList.remove('focus-visible');
    }
    
    // Save preferences
    localStorage.setItem('accessibility-high-contrast', highContrastMode.toString());
    localStorage.setItem('accessibility-font-size', fontSize);
  }, [highContrastMode, fontSize, reducedMotion, focusVisible]);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const { ctrlKey, shiftKey, key, code } = event;
      
      // Quick tools (Ctrl+K)
      if (ctrlKey && key === 'k') {
        event.preventDefault();
        announceToScreenReader('Quick tools opened', 'assertive');
        // Trigger quick tools modal
        const quickToolsEvent = new CustomEvent('open-quick-tools');
        document.dispatchEvent(quickToolsEvent);
        return;
      }
      
      // Cycle focus regions (F6)
      if (key === 'F6') {
        event.preventDefault();
        cycleFocusRegions();
        return;
      }
      
      // Skip to content (Ctrl+Shift+C)
      if (ctrlKey && shiftKey && key === 'c') {
        event.preventDefault();
        skipToContent();
        return;
      }
      
      // Toggle high contrast (Ctrl+Shift+H)
      if (ctrlKey && shiftKey && key === 'h') {
        event.preventDefault();
        const newMode = !highContrastMode;
        setHighContrastMode(newMode);
        announceToScreenReader(
          `High contrast mode ${newMode ? 'enabled' : 'disabled'}`,
          'assertive'
        );
        return;
      }
      
      // Font size controls
      if (ctrlKey && (key === '+' || key === '=' || code === 'Equal')) {
        event.preventDefault();
        const sizes: Array<typeof fontSize> = ['small', 'medium', 'large', 'extra-large'];
        const currentIndex = sizes.indexOf(fontSize);
        if (currentIndex < sizes.length - 1) {
          const newSize = sizes[currentIndex + 1];
          setFontSize(newSize);
          announceToScreenReader(`Font size increased to ${newSize}`, 'polite');
        }
        return;
      }
      
      if (ctrlKey && (key === '-' || key === '_')) {
        event.preventDefault();
        const sizes: Array<typeof fontSize> = ['small', 'medium', 'large', 'extra-large'];
        const currentIndex = sizes.indexOf(fontSize);
        if (currentIndex > 0) {
          const newSize = sizes[currentIndex - 1];
          setFontSize(newSize);
          announceToScreenReader(`Font size decreased to ${newSize}`, 'polite');
        }
        return;
      }
      
      if (ctrlKey && key === '0') {
        event.preventDefault();
        setFontSize('medium');
        announceToScreenReader('Font size reset to medium', 'polite');
        return;
      }
    };
    
    document.addEventListener('keydown', handleKeyDown);
    
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [highContrastMode, fontSize]);

  // Announce to screen reader
  const announceToScreenReader = useCallback((message: string, priority: 'polite' | 'assertive' = 'polite') => {
    screenReaderRef.current?.announce(message, priority);
  }, []);

  // Focus trap management
  const trapFocus = useCallback((element: HTMLElement) => {
    const focusTrap = new FocusTrap(element);
    activeFocusTraps.current.add(focusTrap);
    
    return () => {
      focusTrap.deactivate();
      activeFocusTraps.current.delete(focusTrap);
    };
  }, []);

  // Skip to main content
  const skipToContent = useCallback(() => {
    const mainContent = document.querySelector('[data-focus-region="main-content"]') as HTMLElement;
    if (mainContent) {
      mainContent.focus();
      announceToScreenReader('Skipped to main content', 'polite');
    }
  }, [announceToScreenReader]);

  // Cycle through focus regions
  const cycleFocusRegions = useCallback(() => {
    const nextRegion = (currentFocusRegion + 1) % FOCUS_REGIONS.length;
    const regionSelector = FOCUS_REGIONS[nextRegion];
    const regionElement = document.querySelector(regionSelector) as HTMLElement;
    
    if (regionElement) {
      regionElement.focus();
      const regionName = regionElement.getAttribute('aria-label') || 
                        regionElement.getAttribute('data-region-name') || 
                        `Region ${nextRegion + 1}`;
      announceToScreenReader(`Focused on ${regionName}`, 'polite');
      setCurrentFocusRegion(nextRegion);
    }
  }, [currentFocusRegion, announceToScreenReader]);

  // Enhanced setters with announcements
  const setHighContrastModeWithAnnouncement = useCallback((enabled: boolean) => {
    setHighContrastMode(enabled);
    announceToScreenReader(
      `High contrast mode ${enabled ? 'enabled' : 'disabled'}`,
      'polite'
    );
  }, [announceToScreenReader]);

  const setReducedMotionWithAnnouncement = useCallback((enabled: boolean) => {
    setReducedMotion(enabled);
    announceToScreenReader(
      `Reduced motion ${enabled ? 'enabled' : 'disabled'}`,
      'polite'
    );
  }, [announceToScreenReader]);

  const setFontSizeWithAnnouncement = useCallback((size: typeof fontSize) => {
    setFontSize(size);
    announceToScreenReader(`Font size changed to ${size}`, 'polite');
  }, [announceToScreenReader]);

  // Cleanup focus traps on unmount
  useEffect(() => {
    return () => {
      activeFocusTraps.current.forEach(trap => trap.deactivate());
      activeFocusTraps.current.clear();
    };
  }, []);

  const contextValue: AccessibilityContextType = {
    isScreenReaderActive,
    highContrastMode,
    reducedMotion,
    fontSize,
    keyboardNavigation,
    focusVisible,
    announceToScreenReader,
    setHighContrastMode: setHighContrastModeWithAnnouncement,
    setReducedMotion: setReducedMotionWithAnnouncement,
    setFontSize: setFontSizeWithAnnouncement,
    trapFocus,
    skipToContent,
    cycleFocusRegions,
  };

  return (
    <AccessibilityContext.Provider value={contextValue}>
      {children}
      
      {/* Skip to content link */}
      <a
        href="#main-content"
        className="skip-to-content"
        onClick={(e) => {
          e.preventDefault();
          skipToContent();
        }}
      >
        Skip to main content
      </a>
      
      {/* Accessibility status indicator for development */}
      {process.env.NODE_ENV === 'development' && (
        <div className="accessibility-debug">
          <div>Screen Reader: {isScreenReaderActive ? 'Active' : 'Inactive'}</div>
          <div>High Contrast: {highContrastMode ? 'On' : 'Off'}</div>
          <div>Reduced Motion: {reducedMotion ? 'On' : 'Off'}</div>
          <div>Font Size: {fontSize}</div>
          <div>Keyboard Nav: {keyboardNavigation ? 'Active' : 'Inactive'}</div>
        </div>
      )}
    </AccessibilityContext.Provider>
  );
};

// Accessibility utilities export
export { ColorContrastChecker, FocusTrap, ScreenReaderAnnouncer };

// Accessibility testing utilities
export const AccessibilityTester = {
  /**
   * Run comprehensive accessibility audit
   */
  async runAudit(): Promise<{
    contrastIssues: ReturnType<typeof ColorContrastChecker.analyzePageContrast>;
    focusableElements: HTMLElement[];
    ariaIssues: Array<{ element: HTMLElement; issue: string }>;
    keyboardTraps: HTMLElement[];
  }> {
    const contrastIssues = ColorContrastChecker.analyzePageContrast();
    
    // Find all focusable elements
    const focusableElements = Array.from(
      document.querySelectorAll(
        'a[href], button, textarea, input, select, [tabindex]:not([tabindex="-1"]), [contenteditable="true"]'
      )
    ) as HTMLElement[];
    
    // Check for ARIA issues
    const ariaIssues: Array<{ element: HTMLElement; issue: string }> = [];
    
    // Check for missing alt text on images
    document.querySelectorAll('img').forEach(img => {
      if (!img.alt && !img.getAttribute('aria-label') && !img.getAttribute('aria-labelledby')) {
        ariaIssues.push({ element: img, issue: 'Missing alt text or aria-label' });
      }
    });
    
    // Check for missing form labels
    document.querySelectorAll('input, textarea, select').forEach(input => {
      const htmlInput = input as HTMLInputElement;
      if (!htmlInput.labels?.length && !htmlInput.getAttribute('aria-label') && !htmlInput.getAttribute('aria-labelledby')) {
        ariaIssues.push({ element: htmlInput, issue: 'Missing form label' });
      }
    });
    
    // Check for keyboard traps (simplified)
    const keyboardTraps: HTMLElement[] = [];
    
    return {
      contrastIssues,
      focusableElements,
      ariaIssues,
      keyboardTraps,
    };
  },
  
  /**
   * Generate accessibility report
   */
  async generateReport(): Promise<string> {
    const audit = await this.runAudit();
    
    let report = '# Accessibility Audit Report\n\n';
    
    report += `## Summary\n`;
    report += `- Contrast Issues: ${audit.contrastIssues.length}\n`;
    report += `- ARIA Issues: ${audit.ariaIssues.length}\n`;
    report += `- Focusable Elements: ${audit.focusableElements.length}\n`;
    report += `- Keyboard Traps: ${audit.keyboardTraps.length}\n\n`;
    
    if (audit.contrastIssues.length > 0) {
      report += `## Contrast Issues\n`;
      audit.contrastIssues.forEach((issue, index) => {
        report += `${index + 1}. Ratio: ${issue.ratio.toFixed(2)} - ${issue.recommendation}\n`;
      });
      report += '\n';
    }
    
    if (audit.ariaIssues.length > 0) {
      report += `## ARIA Issues\n`;
      audit.ariaIssues.forEach((issue, index) => {
        report += `${index + 1}. ${issue.issue}\n`;
      });
      report += '\n';
    }
    
    return report;
  }
};