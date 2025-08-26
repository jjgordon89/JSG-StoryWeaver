import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { describe, it, expect } from 'vitest';
import ScreenReaderAnnouncer from '../ScreenReaderAnnouncer';

describe('ScreenReaderAnnouncer', () => {
  it('should render the message correctly', () => {
    const message = 'This is a test message';
    render(<ScreenReaderAnnouncer message={message} />);
    
    const announcer = screen.getByText(message);
    expect(announcer).toBeInTheDocument();
  });

  it('should have the correct aria attributes', () => {
    const message = 'Another test message';
    render(<ScreenReaderAnnouncer message={message} />);
    
    const announcer = screen.getByText(message).parentElement;
    expect(announcer).toHaveAttribute('aria-live', 'polite');
    expect(announcer).toHaveAttribute('aria-atomic', 'true');
  });

  it('should be visually hidden', () => {
    const message = 'A visually hidden message';
    render(<ScreenReaderAnnouncer message={message} />);
    
    const announcer = screen.getByText(message).parentElement;
    expect(announcer).toHaveClass('sr-only');
  });

  it('should update the message when props change', () => {
    const initialMessage = 'Initial message';
    const { rerender } = render(<ScreenReaderAnnouncer message={initialMessage} />);
    
    expect(screen.getByText(initialMessage)).toBeInTheDocument();
    
    const updatedMessage = 'Updated message';
    rerender(<ScreenReaderAnnouncer message={updatedMessage} />);
    
    expect(screen.queryByText(initialMessage)).not.toBeInTheDocument();
    expect(screen.getByText(updatedMessage)).toBeInTheDocument();
  });
});